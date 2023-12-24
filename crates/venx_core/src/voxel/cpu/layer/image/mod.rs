mod slice;
use crate::voxel::{
    cpu::topology::graph::Branch,
    interfaces::{chunk_loader::ChunkLoaderInterface, image::ImageInterface},
};

use self::slice::Slice;

#[derive(Debug)]
pub struct Image {
    slices: Vec<Slice>,
    shared: Vec<Branch>,
}

impl ImageInterface for Image {
    fn insert_segment() {
        todo!()
    }
}

impl ChunkLoaderInterface for Image {
    fn load_chunk(
        &self,
        position: bevy::prelude::UVec3,
        level: u8,
    ) -> Option<crate::chunk::chunk::Chunk> {
        todo!()
    }

    fn load_chunks() {
        todo!()
    }

    fn overlay_chunk() {
        todo!()
    }

    fn overlay_chunks() {
        todo!()
    }
}
