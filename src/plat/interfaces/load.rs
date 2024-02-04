use glam::{UVec3, Vec3, Vec4};

use super::Grid;

pub trait LoadInterface {
    /// Position in chunk grid
    fn load_chunk<const SIZE: usize>(&self, position: UVec3, lod_level: u8) -> Grid<SIZE>;
    fn load_chunks(&self);
    fn overlay_chunk(&self);
    fn overlay_chunks(&self);

    // Mesh creation

    fn compute_mesh_from_chunk<'a, const SIZE: usize>(
        &self,
        chunk: &Grid<SIZE>,
    ) -> &'a [(Vec3, Vec4, Vec3)];
}
