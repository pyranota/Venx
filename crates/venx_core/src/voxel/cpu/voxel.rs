use glam::uvec3;

use crate::{
    chunk::chunk::Chunk,
    plat::Plat,
    voxel::{
        interfaces::{layer::LayerInterface, voxel::VoxelInterface},
        segment::Segment,
    },
};

use super::{
    attribute::tetree::TeTree,
    layer::{canvas::Canvas, Layer},
    topology::graph::Graph,
};

// #[derive(bitcode::Encode, bitcode::Decode)]
#[derive(Debug)]
pub struct Voxel {
    pub layers: Vec<Layer>,
    pub chunk_level: u8,
    pub segment_level: u8,
    pub depth: u8,
}

impl Voxel {
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Voxel {
            chunk_level,
            segment_level,
            depth,
            layers: vec![],
        }
    }
}

impl LayerInterface for Voxel {
    fn new_canvas(&mut self, name: &str) -> usize {
        self.layers.push(Layer::Canvas(Canvas {
            graph: Graph::new(self.depth),
        }));
        self.layers.len() - 1
    }

    fn new_image(&mut self, name: &str) -> usize {
        todo!()
    }

    fn get_image(&self, handle: usize) -> &super::layer::image::Image {
        todo!()
    }

    fn get_image_mut(&mut self, handle: usize) -> &mut super::layer::image::Image {
        todo!()
    }

    fn get_canvas(&self, handle: usize) -> &Canvas {
        todo!()
    }

    fn get_canvas_mut(&mut self, handle: usize) -> &mut Canvas {
        if let Layer::Canvas(canvas) = &mut self.layers[handle] {
            return canvas;
        } else {
            panic!();
        }
    }
}

impl VoxelInterface for Voxel {
    fn load_chunk(
        &self,
        position: glam::UVec3,
        level: u8,
    ) -> std::option::Option<crate::chunk::chunk::Chunk> {
        self.load_chunk(position, level)
    }

    fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunk_n_mesh(&self) {
        todo!()
    }

    fn load_chunks_n_meshes(&self) {
        todo!()
    }

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> super::mesh::Mesh {
        self.to_mesh_naive(chunk)
    }

    fn get(&self, level: u8, position: bevy::prelude::UVec3) -> Option<usize> {
        todo!()
    }

    // fn get(&self, level: u8, position: glam::UVec3) -> Option<usize> {
    //     if let Some(attr_position) = self.topology.get_attr_position(level, position) {
    //         if let Some((block, ..)) = self.attribute.get(attr_position as u32) {
    //             return Some(block as usize);
    //         }
    //         return None;
    //     }
    //     return None;
    // }
}

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
