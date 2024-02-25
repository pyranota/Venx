use glam::{UVec3, Vec3, Vec4};
use venx_core::{
    plat::chunk::chunk::{Chunk, ChunkLoadRequest},
    utils::Grid,
};

use crate::plat::normal::mesh::Mesh;

pub trait LoadInterface {
    /// Position in chunk grid
    /// TODO: Make async
    fn load_chunk(&self, position: UVec3, lod_level: usize, chunk_level: usize) -> Box<Chunk> {
        todo!();
    }
    fn load_chunks(&self, blank_chunks: Box<Vec<ChunkLoadRequest>>) {
        todo!();
    }

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh {
        todo!();
    }
}
