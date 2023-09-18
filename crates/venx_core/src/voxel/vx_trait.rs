use glam::UVec3;

use crate::chunk::chunk::Chunk;

use super::segment::{Segment, SegmentStatic};

pub trait VoxelTrait {
    fn insert_segment<const SIZE: usize>(&mut self, segment: SegmentStatic<SIZE>, position: UVec3);
    fn load_chunk(&self, position: UVec3, level: u8) -> Chunk;
    fn load_chunks(&self, position: UVec3, level: u8) -> Chunk;
    fn load_chunk_n_mesh();
    fn load_chunks_n_meshes();
    fn compute_mesh_from_chunk();
    /*
    fn compute_mesh_turbo();
    fn empty_segment();
    fn load_chunk_non_copy();
    fn load_chunks_non_copy();
    fn insert_segments();
    fn is_segment_empty() -> bool;
    fn merge();
    fn set_voxel();
     */
}
