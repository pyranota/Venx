// pub mod canvas;
pub mod load;
// pub mod image;
pub mod layer;



use downcast_rs::{Downcast};


use self::{layer::LayerInterface, load::LoadInterface};

pub trait PlatInterface: LayerInterface + LoadInterface + Downcast {
    // fn insert_segment(&mut self, segment: Segment, position: UVec3);
    // fn load_chunk(&self, position: UVec3, level: usize) -> Option<Chunk>;
    // fn load_chunks(&self, positions: UVec3, level: usize) -> Chunk;
    // fn load_chunk_n_mesh(&self);

    // fn load_chunks_n_meshes(&self);
    // fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh;
    // fn get(&self, level: usize, position: UVec3) -> Option<usize>;
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
