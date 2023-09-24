use glam::uvec3;

use crate::{
    chunk::chunk::Chunk,
    plat::Plat,
    voxel::{segment::Segment, vx_trait::VoxelTrait},
};

use super::{attribute::tetree::TeTree, topology::graph::Graph};

// #[derive(bitcode::Encode, bitcode::Decode)]
#[derive(Debug)]
pub struct Voxel {
    pub(crate) attribute: TeTree,
    pub topology: Graph,
    pub chunk_level: u8,
    pub segment_level: u8,
}

impl Voxel {
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Voxel {
            attribute: TeTree::new(),
            topology: Graph::new(depth),
            chunk_level,
            segment_level,
        }
    }
}

impl VoxelTrait for Voxel {
    fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
        log::info!("Inserting segment");
        let offset = segment.size() * position;
        segment.iter(|pos, block| {
            // Redo

            if block != 0 {
                let attribute_position = self.topology.set(offset + pos, true);
                dbg!(block, attribute_position);
                self.attribute
                    .insert(attribute_position, 1, (block as i32, 0));
            }
        });
        self.attribute.optimize();
        log::info!("Segment is inserted");
    }

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

    fn get(&self, level: u8, position: glam::UVec3) -> Option<usize> {
        self.topology.get(level, position)
    }
}

#[test]
fn test_insert_segment() {
    let mut plat = Plat::new(5, 2, 4);
    let mut segment = Segment::new(4);
    segment.set(uvec3(15, 0, 11), 11);

    plat.insert_segment(segment, uvec3(0, 0, 0));

    let mut segment = Segment::new(4);
    segment.set(uvec3(0, 5, 0), 15);

    plat.insert_segment(segment, uvec3(0, 1, 0));

    plat.get(0, uvec3(15, 0, 11)).unwrap();
    plat.get(0, uvec3(0, 16 + 5, 0)).unwrap();
    assert_eq!(plat.get(0, uvec3(15, 0, 11) + uvec3(0, 16, 0)), None);
    assert_eq!(plat.get(0, uvec3(0, 0, 0) + uvec3(0, 0, 0)), None);
    assert_eq!(plat.get(0, uvec3(19, 0, 11) + uvec3(16, 16, 0)), None);
}
