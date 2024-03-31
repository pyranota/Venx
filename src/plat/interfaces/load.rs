use async_trait::async_trait;
use glam::UVec3;
use venx_core::plat::chunk::chunk::{Chunk, ChunkLoadRequest};

use crate::plat::normal::mesh::Mesh;

#[async_trait]
pub trait LoadInterface {
    /// Load meshes in generic and most optimal way.
    async fn meshes(&self, _requests: Vec<ChunkLoadRequest>) -> Vec<Mesh> {
        todo!()
    }

    /// Load chunks in generic way.
    async fn chunks(&self, _requests: Vec<ChunkLoadRequest>) -> Box<Vec<Chunk>> {
        todo!()
    }

    /// Converts given chunks to meshes. Each [Mesh] corresponds to [Chunk]
    async fn meshes_from_chunks<'a>(&self, _chunks: Vec<Chunk>) -> Vec<Mesh> {
        todo!();
    }

    // TODO: Make async
    /// Position in chunk grid
    #[deprecated]
    fn load_chunk(&self, _position: UVec3, _lod_level: usize, _chunk_level: usize) -> Box<Chunk> {
        todo!();
    }

    #[deprecated]
    fn load_chunks(&self, _blank_chunks: Box<Vec<ChunkLoadRequest>>) {
        todo!();
    }

    #[deprecated]
    fn compute_mesh_from_chunk<'a>(&self, _chunk: &Chunk) -> Mesh {
        todo!();
    }
}
