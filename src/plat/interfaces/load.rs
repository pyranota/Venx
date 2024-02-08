use glam::{UVec3, Vec3, Vec4};
use venx_core::{plat::chunk::chunk::Chunk, utils::Grid};

use crate::plat::normal::mesh::Mesh;

pub trait LoadInterface {
    /// Position in chunk grid
    fn load_chunk(&self, position: UVec3, lod_level: u8) -> Chunk;
    fn load_chunks(&self);
    fn overlay_chunk(&self);
    fn overlay_chunks(&self);

    // Mesh creation

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh;
}
