use glam::UVec3;

use crate::{chunk::chunk::Chunk, voxel::cpu::mesh::Mesh};

pub trait LoadInterface {
    /// Position in chunk grid
    fn load_chunk(&self, position: UVec3, lod_level: u8) -> Chunk;
    fn load_chunks(&self);
    fn overlay_chunk(&self);
    fn overlay_chunks(&self);

    // Mesh creation

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh;
}
