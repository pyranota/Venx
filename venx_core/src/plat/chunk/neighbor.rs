use glam::{IVec3, UVec3, Vec3};

use crate::voxel::{
    cpu::{utils::lvl_to_size::lvl_to_size, voxel::Voxel},
    interfaces::voxel::VoxelInterface,
};

use super::chunk::Chunk;

impl Voxel {
    pub fn get_neighbor(
        &self,
        chunk: &Chunk,
        local_block_position: impl Into<IVec3>,
        neighbor_direction: impl Into<IVec3>,
    ) -> Option<i32> {
        let real_chunk_size = lvl_to_size(chunk.level());
        let chunk_size = chunk.size();

        let dir: IVec3 = neighbor_direction.into();
        let pos: IVec3 = local_block_position.into();
        let sum = pos + dir;

        if sum.min_element() < 0 {
            for entry in 1..(self.layers[0].graph.entries()) {
                if self.layers[0]
                    .graph
                    .get_node(
                        chunk.lod_level,
                        ((chunk.position * real_chunk_size).as_ivec3() + sum).as_uvec3(),
                        entry,
                    )
                    .is_some()
                {
                    return Some(entry as i32);
                }
            }
            return None;
        } else if sum.max_element() >= chunk_size as i32 {
            for entry in 1..(self.layers[0].graph.entries()) {
                if self.layers[0]
                    .graph
                    .get_node(
                        chunk.lod_level,
                        ((chunk.position * real_chunk_size).as_ivec3() + sum).as_uvec3(),
                        entry,
                    )
                    .is_some()
                {
                    return Some(entry as i32);
                }
            }
            return None;
        } else {
            return chunk.get(sum.as_uvec3());
        }
    }
    pub fn get_neighbor_untyped(
        &self,
        chunk: &Chunk,
        block_position: impl Into<UVec3>,
        neighbor_direction: impl Into<UVec3>,
    ) -> Option<u32> {
        todo!()
    }
}

#[test]
fn neighbor_test() {
    let chunk = Chunk::new(UVec3::ZERO, 0, 4);

    // chunk.get_neighbor((13, 1, 3), (7, 1, 1));
}
