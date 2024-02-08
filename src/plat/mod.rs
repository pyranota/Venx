use std::{
    fs::{create_dir_all, read, read_to_string, File},
    io::{Read, Write},
    ops::Range,
    usize,
};

use anyhow::bail;
use bytes_cast::BytesCast;
use easy_compute::{
    include_spirv, BindGroupBuilder, BufferRW, ComputePassDescriptor, ComputeServer,
    PipelineBuilder,
};
use glam::{uvec3, UVec3, Vec3, Vec4};
use log::info;
use serde::{Deserialize, Serialize};
use venx_core::{
    plat::{chunk::chunk::Chunk, layer::layer::Layer, node::Node, raw_plat::RawPlat},
    utils::Grid,
};

use self::{
    interfaces::{layer::LayerInterface, load::LoadInterface, PlatInterface},
    normal::{cpu_plat::CpuPlat, mesh::Mesh},
    turbo::gpu_plat::GpuPlat,
};

pub mod interfaces;
#[cfg(feature = "mca_converter")]
mod mca_converter;
mod minecraft_blocks;
mod normal;
mod turbo;

pub struct VenxPlat {
    plat: Plat,
}

pub(crate) enum Plat {
    Cpu(CpuPlat),
    #[cfg(feature = "gpu")]
    Gpu(GpuPlat),
}

#[derive(Default, Serialize, Deserialize)]
struct MetaSerDeser {
    depth: u8,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
}

impl VenxPlat {
    /// Save plat to .config directory
    pub fn save(&self, name: &str) -> anyhow::Result<()> {
        info!("Saving {name}.plat");
        let path = ".cache/".to_owned() + name;
        // TODO: Make .config in custom location
        create_dir_all(format!("{}.plat", path))?;
        create_dir_all(format!("{}.plat/layers/", path))?;
        //   let entry: String = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())?;
        let mut file = File::create(format!("{}.plat/meta.ron", path))?;

        let raw_plat = self.get_normal_unchecked().borrow_raw_plat();
        let meta: String = ron::ser::to_string_pretty(
            &MetaSerDeser {
                depth: raw_plat.depth,
                position: (0., 0., 0.),
                rotation: (0., 0., 0.),
            },
            ron::ser::PrettyConfig::default(),
        )?;
        file.write_all(meta.as_bytes())?;
        // Create layers dirs

        for (layer_name, layer) in raw_plat.layers() {
            let layer_path = format!("{path}.plat/layers/{layer_name}");

            create_dir_all(&layer_path)?;

            // let level_stringified: String =
            //     ron::ser::to_string_pretty(&level, ron::ser::PrettyConfig::default())?;
            let encoded_entries: Vec<u8> = bitcode::encode(layer.entries).unwrap();
            let encoded_nodes: Vec<u8> = bitcode::encode(layer.nodes).unwrap();

            let mut entries_file = File::create(format!("{}/entries", layer_path))?;
            entries_file.write_all(&encoded_entries)?;

            let mut nodes_file = File::create(format!("{}/nodes", layer_path))?;
            nodes_file.write_all(&encoded_nodes)?;
        }
        Ok(())
    }

    pub fn load(name: &str) -> anyhow::Result<Self> {
        info!("Loading {name}.plat");
        let path = ".cache/".to_owned() + name;
        let meta: MetaSerDeser = ron::from_str(&read_to_string(format!("{path}.plat/meta.ron"))?)?;

        let mut components = [
            (vec![], vec![]),
            (vec![], vec![]),
            (vec![], vec![]),
            (vec![], vec![]),
        ];

        for (i, layer_name) in ["base", "tmp", "schem", "canvas"].iter().enumerate() {
            let entries_path = format!("{path}.plat/layers/{layer_name}/entries");
            let nodes_path = format!("{path}.plat/layers/{layer_name}/nodes");

            let entries: Vec<usize> = bitcode::decode(&read(entries_path)?)?;
            let nodes: Vec<Node> = bitcode::decode(&read(nodes_path)?)?;

            components[i] = (nodes, entries);
        }

        // TODO: remove bottleneck. Handle this without cloning
        Ok(VenxPlat {
            plat: Plat::Cpu(CpuPlat::from_existing(
                meta.depth,
                5,
                5,
                components[0].clone(),
                components[1].clone(),
                components[2].clone(),
                components[3].clone(),
            )),
        })
    }

    pub fn get_normal_unchecked(&self) -> &CpuPlat {
        match &self.plat {
            Plat::Cpu(normal) => normal,
            Plat::Gpu(_) => panic!("Trying to get normal plat while it is turbo"),
        }
    }
    pub fn get_turbo_unchecked(&mut self) -> &mut GpuPlat {
        match &mut self.plat {
            Plat::Cpu(_) => panic!("Trying to get turbo plat while it is normal"),
            Plat::Gpu(turbo) => turbo,
        }
    }
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let plat = Plat::Cpu(CpuPlat::new_plat(depth, chunk_level, segment_level));

        VenxPlat { plat: plat }
    }
    /// tmp
    pub(crate) fn new_with_length(
        depth: u8,
        chunk_level: u8,
        segment_level: u8,
        len: usize,
    ) -> Self {
        let plat = Plat::Cpu(CpuPlat::new_plat_with_length(
            depth,
            chunk_level,
            segment_level,
            len,
        ));

        VenxPlat { plat: plat }
    }
    /// Get depth and verify that its synced
    pub fn depth(&mut self) -> u8 {
        match &mut self.plat {
            Plat::Cpu(cpu_plat) => {
                let plat = cpu_plat.borrow_raw_plat();
                let plat_depth = plat.depth;

                assert_eq!(plat.base.depth, plat_depth);
                assert_eq!(plat.tmp.depth, plat_depth);
                assert_eq!(plat.schem.depth, plat_depth);
                assert_eq!(plat.canvas.depth, plat_depth);

                plat_depth
            }
            Plat::Gpu(_) => todo!("You cant get depth from plat on gpu, yet"),
        }
    }
    /// Depth, chunk_level, segment_level
    pub async fn new_turbo(depth: u8, chunk_level: u8, segment_level: u8) -> VenxPlat {
        VenxPlat {
            plat: Plat::Gpu(GpuPlat::new_plat(depth, chunk_level, segment_level).await),
        }
    }
    pub async fn transfer_to_gpu(self) -> Self {
        VenxPlat {
            plat: match self.plat {
                Plat::Cpu(cpu_plat) => Plat::Gpu(cpu_plat.transfer_to_gpu().await),
                Plat::Gpu(_) => panic!("It is dumb idea to transfer data from gpu to gpu"),
            },
        }
    }
    pub async fn transfer_from_gpu(self) -> Self {
        VenxPlat {
            plat: match self.plat {
                Plat::Cpu(_) => panic!("It is dumb idea to transfer data from cpu to cpu"),
                Plat::Gpu(gpu_plat) => Plat::Cpu(gpu_plat.transfer_from_gpu().await),
            },
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
        lod: Option<u8>,
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
                    // let mut lod_level = (u32::max(z, x) / 128) as u8;

                    // if lod_level > 2 {
                    //     lod_level = 2;
                    // }

                    // lod_level = 0;

                    // TODO: Make LOD's work
                    let chunk = plat.load_chunk(uvec3(x, y, z), lod.unwrap_or(0));

                    let vx_mesh = plat.compute_mesh_from_chunk(&chunk);

                    let mesh_idx = counter / capacity;

                    'mesh: for (pos, color, normal) in vx_mesh.iter() {
                        // Each returned mesh is static length, so not all attributes in that mesh are used
                        // To prevent leaking zero attributes into actual mesh, we check it
                        // Dont create blocks with color Vec4::ZERO, it will break the mesh
                        if color.to_array() == glam::f32::Vec4::ZERO.to_array() {
                            break 'mesh;
                        }

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
    fn load_chunk(&self, position: glam::UVec3, lod_level: u8) -> Box<Chunk> {
        match &self.plat {
            Plat::Cpu(plat) => plat.load_chunk(position, lod_level),
            Plat::Gpu(plat) => plat.load_chunk(position, lod_level),
        }
    }

    fn load_chunks(&self) {
        todo!()
    }

    fn overlay_chunk(&self) {
        todo!()
    }

    fn overlay_chunks(&self) {
        todo!()
    }

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh {
        match &self.plat {
            Plat::Cpu(plat) => plat.compute_mesh_from_chunk(chunk),
            Plat::Gpu(plat) => plat.compute_mesh_from_chunk(chunk),
        }
    }
}

impl LayerInterface for VenxPlat {
    fn set_segment<const SIZE: usize>(
        &mut self,
        layer: usize,
        segment: Grid<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        match &mut self.plat {
            Plat::Cpu(ref mut plat) => plat.set_voxel(layer, position, ty),
            Plat::Gpu(ref mut plat) => plat.set_voxel(layer, position, ty),
        }
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{interfaces::layer::LayerInterface, VenxPlat};

    #[test]
    fn transfer() {
        // Create 2 identical plats

        // First
        let mut normal_plat_1 = VenxPlat::new(12, 5, 9);
        // Build something
        normal_plat_1.set_voxel(0, (4, 4, 4).into(), 1);
        normal_plat_1.set_voxel(0, (4, 5, 4).into(), 1);
        normal_plat_1.set_voxel(0, (5, 5, 5).into(), 2);

        // Second
        let mut normal_plat_2 = VenxPlat::new(12, 5, 9);
        // Build something
        normal_plat_2.set_voxel(0, (4, 4, 4).into(), 1);
        normal_plat_2.set_voxel(0, (4, 5, 4).into(), 1);
        normal_plat_2.set_voxel(0, (5, 5, 5).into(), 2);

        // Transfer first to gpu
        let turbo_plat = pollster::block_on(normal_plat_1.transfer_to_gpu());

        // Transfer back to cpu
        let mut transfered_from_gpu = pollster::block_on(turbo_plat.transfer_from_gpu());

        // Compare

        assert_eq!(
            normal_plat_2.get_normal_unchecked().borrow_raw_plat(),
            transfered_from_gpu.get_normal_unchecked().borrow_raw_plat()
        );
    }
}
