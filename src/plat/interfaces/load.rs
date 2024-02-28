use glam::{UVec3};
use venx_core::{
    plat::chunk::chunk::{Chunk, ChunkLoadRequest},
};

use crate::plat::normal::mesh::Mesh;

pub trait LoadInterface {
    /// Position in chunk grid
    /// TODO: Make async
    fn load_chunk(&self, _position: UVec3, _lod_level: usize, _chunk_level: usize) -> Box<Chunk> {
        todo!();
    }
    fn load_chunks(&self, _blank_chunks: Box<Vec<ChunkLoadRequest>>) {
        todo!();
    }

    fn compute_mesh_from_chunk<'a>(&self, _chunk: &Chunk) -> Mesh {
        todo!();
    }
}
