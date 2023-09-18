use crate::voxel::vx_trait::VoxelTrait;

use super::{attribute::tetree::TeTree, topology::graph::Graph};

pub struct Voxel {
    pub(crate) attribute: TeTree,
    pub topology: Graph,
}

impl Voxel {
    pub fn new(depth: u8) -> Self {
        Voxel {
            attribute: TeTree { nodes: vec![] },
            topology: Graph::new(depth),
        }
    }
}

// impl VoxelTrait for Voxel {
//     fn insert_segment(segment: crate::voxel::segment::Segment, position: glam::UVec3) {
//         todo!()
//     }

//     fn load_chunk(position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
//         todo!()
//     }

//     fn load_chunks(position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
//         todo!()
//     }

//     fn load_chunk_n_mesh() {
//         todo!()
//     }

//     fn load_chunks_n_meshes() {
//         todo!()
//     }

//     fn compute_mesh_from_chunk() {
//         todo!()
//     }
// }
