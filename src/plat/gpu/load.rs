use venx_core::{plat::chunk::chunk::Chunk, utils::Grid};

use crate::plat::interfaces::load::LoadInterface;

use super::gpu_plat::GpuPlat;

impl LoadInterface for GpuPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: u8) -> Chunk {
        todo!()
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

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> crate::plat::cpu::mesh::Mesh {
        todo!()
    }
}
