use std::usize;

use easy_compute::{
    include_spirv, BindGroupBuilder, BufferRW, ComputePassDescriptor, ComputeServer,
    PipelineBuilder,
};
use glam::{UVec3, Vec3, Vec4};
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

impl VenxPlat {
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let plat = Plat::Cpu(CpuPlat::new_plat(depth, chunk_level, segment_level));

        VenxPlat { plat: plat }
    }
    /// Get depth and verify that its synced
    pub fn depth(&self) -> u8 {
        match &self.plat {
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
}

impl PlatInterface for VenxPlat {}

impl LoadInterface for VenxPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: u8) -> Chunk {
        match &self.plat {
            Plat::Cpu(ref plat) => plat.load_chunk(position, lod_level),
            Plat::Gpu(ref plat) => plat.load_chunk(position, lod_level),
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
            Plat::Cpu(ref plat) => plat.compute_mesh_from_chunk(chunk),
            Plat::Gpu(ref plat) => plat.compute_mesh_from_chunk(chunk),
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

    fn get_voxel(&self, position: glam::UVec3) -> Option<(usize, usize)> {
        todo!()
    }
}
