use glam::UVec3;

use crate::chunk::chunk::Chunk;

pub trait ChunkLoaderInterface {
    fn load_chunk(&self, position: UVec3, level: u8) -> Option<Chunk>;
    fn load_chunks();
    fn overlay_chunk();
    fn overlay_chunks();
}
