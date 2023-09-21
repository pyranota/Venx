use crate::voxel::vx_trait::VoxelTrait;

use super::{attribute::tetree::TeTree, topology::graph::Graph};

// #[derive(bitcode::Encode, bitcode::Decode)]
pub struct Voxel {
    pub(crate) attribute: TeTree,
    pub topology: Graph,
    pub chunk_level: u8,
    pub segment_level: u8,
}

impl Voxel {
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Voxel {
            attribute: TeTree { nodes: vec![] },
            topology: Graph::new(depth),
            chunk_level,
            segment_level,
        }
    }
}

impl VoxelTrait for Voxel {
    fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
        
        todo!()
    }

    fn load_chunk(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunk_n_mesh() {
        todo!()
    }

    fn load_chunks_n_meshes() {
        todo!()
    }

    fn compute_mesh_from_chunk() {
        todo!()
    }
}
