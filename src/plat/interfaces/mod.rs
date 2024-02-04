// pub mod canvas;
pub mod load;
// pub mod image;
pub mod layer;

use std::{any::Any, fmt::Debug};

use downcast_rs::{impl_downcast, Downcast};
use glam::UVec3;

use self::{layer::LayerInterface, load::LoadInterface};

pub type Grid<const SIZE: usize> = [[[u32; SIZE]; SIZE]; SIZE];

pub trait PlatInterface: Debug + LayerInterface + LoadInterface + Downcast {
    // fn insert_segment(&mut self, segment: Segment, position: UVec3);
    // fn load_chunk(&self, position: UVec3, level: u8) -> Option<Chunk>;
    // fn load_chunks(&self, positions: UVec3, level: u8) -> Chunk;
    // fn load_chunk_n_mesh(&self);

    // fn load_chunks_n_meshes(&self);
    // fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh;
    // fn get(&self, level: u8, position: UVec3) -> Option<usize>;
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
