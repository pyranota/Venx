use glam::UVec3;

use crate::chunk::chunk::Chunk;

use super::segment::Segment;

pub trait VoxelTrait {
    fn insert_segment(segment: Segment, position: UVec3);
    fn load_chunk(position: UVec3, level: u8) -> Chunk;
    fn load_chunks(position: UVec3, level: u8) -> Chunk;
    fn load_chunk_n_mesh();
    fn load_chunks_n_meshes();
    fn compute_mesh_from_chunk();
    // async fn load_chunks();
    // fn load_chunk_non_copy();
    // fn load_chunks_non_copy();
    // async fn insert_segments();
    /*
    fn merge();
    fn set_voxel();
     */
}
