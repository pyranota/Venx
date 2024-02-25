use venx_core::plat::{
    chunk::chunk::{Chunk, ChunkLoadRequest},
    layer::layer::Lr,
};

use crate::plat::interfaces::load::LoadInterface;

use super::{cpu_plat::CpuPlat, mesh::Mesh};

impl LoadInterface for CpuPlat {
    fn load_chunk(
        &self,
        position: glam::UVec3,
        lod_level: usize,
        chunk_level: usize,
    ) -> Box<Chunk> {
        Box::new(self.borrow_raw_plat()[Lr::BASE].load_chunk(
            position.to_array().into(),
            lod_level,
            chunk_level,
        ))
    }

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh {
        self.to_mesh_greedy(chunk)
    }
}
