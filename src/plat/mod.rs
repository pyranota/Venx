use std::usize;

use glam::{UVec3, Vec3, Vec4};
use venx_core::{
    plat::{chunk::chunk::Chunk, layer::layer::Layer, raw_plat::RawPlat},
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

#[derive(Debug)]
pub struct VenxPlat {
    plat: Plat,
}

#[derive(Debug)]
pub(crate) enum Plat {
    Cpu(CpuPlat),
    #[cfg(feature = "gpu")]
    Gpu(GpuPlat),
}

impl VenxPlat {
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let plat = Plat::Cpu(CpuPlat {
            raw_plat: RawPlat {
                //controller: Controller::new(depth, chunk_level, segment_level),
                position: (0, 0, 0),
                rotation: (0, 0, 0),
                depth,
                base: Layer::new::<1_280_000>(depth),
                tmp: Layer::new::<128_000>(depth),
                schem: Layer::new::<128_000>(depth),
                canvas: Layer::new::<128_000>(depth),
                //chunks: ChunksStorage {},
            },
        });

        VenxPlat { plat: plat }
    }

    // fn inner_plat(&self) -> Box<dyn PlatInterface> {
    //     Box::new(match self.plat {
    //         Plat::Cpu(plat) => plat,
    //         Plat::Gpu(plat) => plat,
    //     })
    // }
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
