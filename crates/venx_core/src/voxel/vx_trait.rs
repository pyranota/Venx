use std::fmt::Debug;

use glam::UVec3;

use crate::chunk::chunk::Chunk;

use super::{
    cpu::mesh::Mesh,
    segment::{Segment, SegmentStatic},
};

pub trait VoxelTrait: Debug {
    fn insert_segment(&mut self, segment: Segment, position: UVec3);
    fn load_chunk(&self, position: UVec3, level: u8) -> Option<Chunk>;
    fn load_chunks(&self, position: UVec3, level: u8) -> Chunk;
    fn load_chunk_n_mesh(&self);
    fn load_chunks_n_meshes(&self);
    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh;
    fn get(&self, level: u8, position: UVec3) -> Option<usize>;
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
