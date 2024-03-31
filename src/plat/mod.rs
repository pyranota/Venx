use std::{
    collections::HashMap,
    fs::{create_dir_all, read, read_to_string, File},
    io::Write,
    ops::Range,
    usize,
};

use async_trait::async_trait;
use glam::{uvec3, Quat, UVec3, Vec3, Vec4};
use log::info;
use serde::{Deserialize, Serialize};
use venx_core::plat::{
    chunk::chunk::Chunk,
    node::Node,
    node_l2::NodeL2,
    op::get::GetNodeResult,
    raw_plat::LayerIndex::{Base, Canvas, Schem, Tmp},
};

use self::{
    block_collections::smbc::SMBC,
    interfaces::{layer::LayerInterface, load::LoadInterface, PlatInterface},
    loader::VenxLoader,
    normal::{cpu_plat::CpuPlat, mesh::Mesh},
};

mod block_collections;
mod charts;
pub mod fs;

pub mod interfaces;
pub mod loader;
#[cfg(feature = "mca_converter")]
mod mca_converter;
mod minecraft_blocks;
pub mod normal;
#[cfg(feature = "turbo")]
pub mod turbo;

// TODO: Get rid of [VenxPlat] and just use [Plat]
pub struct VenxPlat {
    plat: Plat,
    loader: VenxLoader,
    smbcs: Vec<SMBC>,
}

pub(crate) enum Plat {
    Cpu(CpuPlat),
    #[cfg(feature = "turbo")]
    Gpu(GpuPlat),
}

#[derive(Default, Serialize, Deserialize)]
struct MetaSerDeser {
    depth: usize,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
}

impl VenxPlat {
    pub fn get_normal_unchecked(&self) -> &CpuPlat {
        match &self.plat {
            Plat::Cpu(normal) => normal,
            #[cfg(feature = "turbo")]
            Plat::Gpu(_) => panic!("Trying to get normal plat while it is turbo"),
        }
    }
    #[cfg(feature = "turbo")]
    pub fn get_turbo_unchecked(&mut self) -> &mut GpuPlat {
        match &mut self.plat {
            Plat::Cpu(_) => panic!("Trying to get turbo plat while it is normal"),
            #[cfg(feature = "turbo")]
            Plat::Gpu(turbo) => turbo,
        }
    }
    /// Depth, chunk_level, segment_level
    pub fn new(depth: usize, chunk_level: usize, segment_level: usize) -> Self {
        let plat = Plat::Cpu(CpuPlat::new_plat(depth, chunk_level, segment_level));
        //let loader = VenxLoader::new(initial_focus, bucket_size, bucket_amount, indirect_buffer, vertex_buffer)

        VenxPlat {
            plat,
            loader: todo!(),
            smbcs: todo!(),
        }
    }

    /// Get depth and verify that its synced
    pub fn depth(&self) -> usize {
        match &self.plat {
            Plat::Cpu(cpu_plat) => {
                let plat = cpu_plat.borrow_raw_plat();
                let plat_depth = plat.depth;

                assert_eq!(plat[Base].depth, plat_depth);
                assert_eq!(plat[Tmp].depth, plat_depth);
                assert_eq!(plat[Schem].depth, plat_depth);
                assert_eq!(plat[Canvas].depth, plat_depth);

                plat_depth
            }
            #[cfg(feature = "turbo")]
            Plat::Gpu(_) => todo!("You cant get depth from plat on gpu, yet"),
        }
    }
    #[cfg(feature = "turbo")]
    /// Depth, chunk_level, segment_level
    pub async fn new_turbo(depth: usize, chunk_level: usize, segment_level: usize) -> VenxPlat {
        VenxPlat {
            plat: Plat::Gpu(GpuPlat::new_plat(depth, chunk_level, segment_level).await),
            loader: todo!(),
            smbcs: todo!(),
        }
    }
    #[cfg(feature = "turbo")]
    /// Will return Normal version if there is not enough free memory on GPU.
    ///
    /// So it does not guarantee transfer on GPU
    pub async fn transfer_to_gpu(self) -> Self {
        VenxPlat {
            plat: match self.plat {
                Plat::Cpu(cpu_plat) => Plat::Gpu(cpu_plat.transfer_to_gpu().await),
                Plat::Gpu(_) => panic!("It is dumb idea to transfer data from gpu to gpu"),
            },
            loader: self.loader,
            smbcs: self.smbcs,
        }
    }
    #[cfg(feature = "turbo")]
    pub async fn transfer_from_gpu(self) -> Self {
        VenxPlat {
            plat: match self.plat {
                Plat::Cpu(_) => panic!("It is dumb idea to transfer data from cpu to cpu"),
                Plat::Gpu(gpu_plat) => Plat::Cpu(gpu_plat.transfer_from_gpu().await),
            },
            loader: self.loader,
            smbcs: self.smbcs,
        }
    }

    /// Load meshes for given chunks. Used for debug purposes and examples
    /// Its block-on operation
    /// Returns Vec of (Vertices, Colors, Normals)
    pub fn static_mesh(
        &self,
        chunk_range_x: Range<u32>,
        chunk_range_y: Range<u32>,
        chunk_range_z: Range<u32>,
        _lod: Option<usize>,
    ) -> Vec<(Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<[f32; 3]>)> {
        let chunks_amount = (chunk_range_x.end - chunk_range_x.start)
            * (chunk_range_z.end - chunk_range_z.start)
            * (chunk_range_y.end - chunk_range_y.start);

        // Basically amount of inidvidual 3d models / draw-calls
        let meshes_amount = 128;
        // How many vertices can be in single mesh
        let capacity = 500 * chunks_amount as usize;
        let mut meshes = vec![
            // Single mesh
            (
                Vec::<[f32; 3]>::with_capacity(capacity),
                Vec::<[f32; 4]>::with_capacity(capacity),
                Vec::<[f32; 3]>::with_capacity(capacity)
            );
            meshes_amount
        ];

        log::info!("Loading chunks and computing meshes");

        // Global amount of vertices. Used to determine which mesh should be written
        let mut counter = 0;
        let plat = self;

        // TODO: Make use of `load_chunks` to speed up calculations with turbo mode enabled
        for x in chunk_range_x.clone() {
            info!(
                "Progress: {}/{}",
                x - chunk_range_x.start,
                chunk_range_x.end - chunk_range_x.start
            );
            for z in chunk_range_z.clone() {
                for y in chunk_range_y.clone() {
                    let mut lod_level = ((u32::max(z, x) as f64).sqrt() as u32 / (4)) as usize;

                    if lod_level > 2 {
                        lod_level = 2;
                        let _ = lod_level;
                    }

                    // lod_level = 0;
                    //let mut lod = lod;

                    // let fetched_chunk = plat.load_chunk(uvec3(x, y, z), 0, 5);
                    // let mut found = false;
                    // fetched_chunk.iter(|p, ty| {
                    //     if ty == 8 && lod_level > 0 && !found {
                    //         found = true;
                    //         lod_level -= 1;
                    //         return;
                    //     }
                    // });

                    let chunk = plat.load_chunk(uvec3(x, y, z), 0, 5);

                    let vx_mesh = plat.compute_mesh_from_chunk(&chunk);

                    let mesh_idx = counter / capacity;

                    let mut count = 0;

                    'mesh: for attr in vx_mesh.iter() {
                        let (pos, color, normal) = (
                            Vec3::from_slice(&attr[0..3]),
                            Vec4::from_slice(&attr[3..7]),
                            Vec3::from_slice(&attr[7..10]),
                        );

                        // Each returned mesh is static length, so not all attributes in that mesh are used
                        // To prevent leaking zero attributes into actual mesh, we check it
                        // Dont create blocks with color Vec4::ZERO, it will break the mesh
                        if color.to_array() == glam::f32::Vec4::ZERO.to_array() {
                            if count != 0 {
                                dbg!(count / 6);
                            }

                            break 'mesh;
                        }

                        count += 1;

                        counter += 1;
                        meshes[mesh_idx].0.push(pos.to_array());
                        meshes[mesh_idx].1.push(color.to_array());
                        meshes[mesh_idx].2.push(normal.to_array());
                    }
                }
            }
        }

        meshes
    }
}

impl PlatInterface for VenxPlat {}

impl LoadInterface for VenxPlat {
    fn load_chunk(
        &self,
        position: glam::UVec3,
        lod_level: usize,
        chunk_level: usize,
    ) -> Box<Chunk> {
        match &self.plat {
            Plat::Cpu(plat) => plat.load_chunk(position, lod_level, chunk_level),
            #[cfg(feature = "turbo")]
            Plat::Gpu(plat) => plat.load_chunk(position, lod_level, chunk_level),
        }
    }

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh {
        match &self.plat {
            Plat::Cpu(plat) => plat.compute_mesh_from_chunk(chunk),
            #[cfg(feature = "turbo")]
            Plat::Gpu(plat) => plat.compute_mesh_from_chunk(chunk),
        }
    }

    fn load_chunks(&self, blank_chunks: Box<Vec<venx_core::plat::chunk::chunk::ChunkLoadRequest>>) {
        match &self.plat {
            Plat::Cpu(_plat) => todo!(),
            #[cfg(feature = "turbo")]
            Plat::Gpu(plat) => plat.load_chunks(blank_chunks),
        }
    }
}
#[async_trait]
impl LayerInterface for VenxPlat {
    async fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        match &mut self.plat {
            Plat::Cpu(ref mut plat) => plat.set_voxel(layer, position, ty),
            #[cfg(feature = "turbo")]
            Plat::Gpu(ref mut plat) => plat.set_voxel(layer, position, ty),
        };
    }

    fn compress(
        &mut self,
        layer: usize,
        position: UVec3,
        level: u32,
        lookup_tables: &mut Vec<std::collections::HashMap<venx_core::plat::node::Node, usize>>,
        lookup_table_l2: &mut HashMap<NodeL2, usize>,
    ) {
        info!("Compress");
        match &mut self.plat {
            Plat::Cpu(plat) => {
                plat.compress(layer, position, level, lookup_tables, lookup_table_l2)
            }
            #[cfg(feature = "turbo")]
            Plat::Gpu(_plat) => todo!(),
        }
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<GetNodeResult> {
        match &self.plat {
            Plat::Cpu(plat) => plat.get_voxel(position),
            #[cfg(feature = "turbo")]
            Plat::Gpu(_plat) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pollster::block_on;

    use super::{interfaces::layer::LayerInterface, VenxPlat};

    #[cfg(feature = "turbo")]
    #[test]
    fn transfer() {
        // Create 2 identical plats
        let future = async {
            // First
            let mut normal_plat_1 = VenxPlat::new(6, 5, 9);
            // Build something
            normal_plat_1.set_voxel(0, (4, 4, 4).into(), 1).await;
            normal_plat_1.set_voxel(0, (4, 5, 4).into(), 1).await;
            normal_plat_1.set_voxel(0, (5, 5, 5).into(), 2).await;

            // Second
            let mut normal_plat_2 = VenxPlat::new(6, 5, 9);
            // Build something
            normal_plat_2.set_voxel(0, (4, 4, 4).into(), 1).await;
            normal_plat_2.set_voxel(0, (4, 5, 4).into(), 1).await;
            normal_plat_2.set_voxel(0, (5, 5, 5).into(), 2).await;

            // Transfer first to gpu
            let turbo_plat = pollster::block_on(normal_plat_1.transfer_to_gpu());

            // Transfer back to cpu
            let transfered_from_gpu = pollster::block_on(turbo_plat.transfer_from_gpu());

            // Compare

            assert_eq!(
                normal_plat_2.get_normal_unchecked().borrow_raw_plat(),
                transfered_from_gpu.get_normal_unchecked().borrow_raw_plat()
            );
        };

        block_on(future);
    }
}
