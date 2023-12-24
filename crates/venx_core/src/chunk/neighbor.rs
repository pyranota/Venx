use glam::{IVec3, UVec3, Vec3};

use crate::voxel::{cpu::voxel::Voxel, interfaces::voxel::VoxelInterface};

use super::chunk::Chunk;

impl Voxel {
    pub fn get_neighbor(
        &self,
        chunk: &Chunk,
        local_block_position: impl Into<IVec3>,
        neighbor_direction: impl Into<IVec3>,
    ) -> Option<i32> {
        let chunk_size = chunk.size();
        let dir: IVec3 = neighbor_direction.into();
        let pos: IVec3 = local_block_position.into();
        let sum = pos + dir;
        if sum.min_element() < 0 {
            // if self
            //     .get(
            //         chunk.lod_level,
            //         ((chunk.position * chunk_size).as_ivec3() + sum).as_uvec3(),
            //     )
            //     .is_some()
            // {
            //     return Some(1);
            // }
            return None;
        } else if sum.max_element() >= chunk_size as i32 {
            // if self
            //     .get(
            //         chunk.lod_level,
            //         ((chunk.position * chunk_size).as_ivec3() + sum).as_uvec3(),
            //     )
            //     .is_some()
            // {
            //     return Some(1);
            // }
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
