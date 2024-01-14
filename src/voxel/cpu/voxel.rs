use glam::uvec3;

use crate::{
    chunk::chunk::Chunk,
    plat::Plat,
    voxel::{
        interfaces::{layer::LayerInterface, voxel::VoxelInterface},
        segment::Segment,
    },
};

use super::{layer::VXLayer, topology::graph::Graph};

// #[derive(bitcode::Encode, bitcode::Decode)]
#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Voxel {
    raw_plat: RawPlat
}

impl Voxel {
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Voxel {
            chunk_level,
            segment_level,
            depth,
            layers: vec![VXLayer::new(depth)],
        }
    }
}

impl VoxelInterface for Voxel {}

// #[test]
// fn test_insert_segment() {
//     let mut plat = Plat::new(5, 2, 4);
//     let mut segment = Segment::new(4);
//     segment.set(uvec3(15, 0, 11), 11);

//     plat.insert_segment(segment, uvec3(0, 0, 0));

//     let mut segment = Segment::new(4);
//     segment.set(uvec3(0, 5, 0), 15);

//     plat.insert_segment(segment, uvec3(0, 1, 0));

//     plat.get(0, uvec3(15, 0, 11)).unwrap();
//     plat.get(0, uvec3(0, 16 + 5, 0)).unwrap();
//     assert_eq!(plat.get(0, uvec3(15, 0, 11) + uvec3(0, 16, 0)), None);
//     assert_eq!(plat.get(0, uvec3(0, 0, 0) + uvec3(0, 0, 0)), None);
//     assert_eq!(plat.get(0, uvec3(19, 0, 11) + uvec3(16, 16, 0)), None);
// }
