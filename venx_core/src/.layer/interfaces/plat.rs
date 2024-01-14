use glam::UVec3;

pub trait PlatExt {
    /// Get voxel at given position
    fn get_voxel(&self, position: UVec3) -> Option<usize>;

    /// Set voxel as regular user
    fn set_voxel(&mut self, position: UVec3, block_type: usize);

    fn set_voxel_layered(&mut self, layer: usize, position: UVec3, block_type: usize);

    fn set_segment(&mut self, layer: usize, segment: bool, position: UVec3);

    fn merge_segment(&mut self, layer: usize, segment: bool, position: UVec3);
}
