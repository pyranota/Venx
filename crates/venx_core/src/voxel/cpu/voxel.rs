use crate::voxel::vx_trait::VoxelTrait;

use super::{attribute::tetree::TeTree, topology::graph::Graph};

pub struct Voxel {
    pub attribute: TeTree,
    pub topology: Graph,
}

impl VoxelTrait for Voxel {
    fn insert_segment(segment: crate::voxel::segment::Segment, position: glam::UVec3) {
        todo!()
    }

    fn load_chunk(position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }
}
