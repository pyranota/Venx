use spirv_std::glam::IVec3;

use crate::{plat::raw_plat::RawPlat, utils::l2s};

use super::chunk::Chunk;

// GLOBAL TODO:
// Dude, it just needs to be reworked. Yet again...
// This time it needs to be able to get neighbor without plat.
impl Chunk {
    pub fn get_neighbor(
        &self,
        plat: &RawPlat,
        local_block_position: impl Into<IVec3>,
        neighbor_direction: impl Into<IVec3>,
    ) -> Option<u32> {
        let real_chunk_size = l2s(self.chunk_level());
        let chunk_size = self.size();

        let dir: IVec3 = neighbor_direction.into();
        let pos: IVec3 = local_block_position.into();
        let sum = pos + dir;

        // TODO: Make better
        // Now its just saying that there is no one on border if chunk lod_level is above 0
        // It forces to draw every single border
        // It should do that only on borders with chunks on lover lod_level
        // Working on CPU not working on GPU:
        // if
        // // self.lod_level() == 0 &&
        // (sum.min_element() < 0 || sum.max_element() >= chunk_size as i32) {
        //     let res = plat.get_node(
        //         ((self.position() * real_chunk_size).as_ivec3() + sum).as_uvec3(),
        //         self.lod_level(),
        //     );

        //     if res.is_some() {
        //         return Some(res.voxel_id as u32);
        //     }

        //     None
        // } else {
        //     return self.get(sum.as_uvec3());
        // }

        if
        // self.lod_level() == 0 &&
        sum.z < 0
            || sum.y < 0
            || sum.x < 0
            || sum.x >= chunk_size as i32
            || sum.y >= chunk_size as i32
            || sum.z >= chunk_size as i32
        {
            let res = plat.get_node(
                ((self.position() * real_chunk_size).as_ivec3() + sum).as_uvec3(),
                self.lod_level(),
            );

            if res.is_some() {
                return Some(res.voxel_id as u32);
            }

            None
        } else {
            return self.get(sum.as_uvec3());
        }
    }

    pub fn get_neighbor_unchecked(
        &self,
        _plat: &RawPlat,
        local_block_position: IVec3,
        neighbor_direction: IVec3,
    ) -> u32 {
        let _real_chunk_size = l2s(self.chunk_level());
        let chunk_size = self.size();

        let sum = neighbor_direction + local_block_position;

        // TODO: Make better
        // Now its just saying that there is no one on border if chunk lod_level is above 0
        // It forces to draw every single border
        // It should do that only on borders with chunks on lover lod_level
        // Working on CPU not working on GPU:
        // if
        // // self.lod_level() == 0 &&
        // (sum.min_element() < 0 || sum.max_element() >= chunk_size as i32) {
        //     let res = plat.get_node(
        //         ((self.position() * real_chunk_size).as_ivec3() + sum).as_uvec3(),
        //         self.lod_level(),
        //     );

        //     if res.is_some() {
        //         return Some(res.voxel_id as u32);
        //     }

        //     None
        // } else {
        //     return self.get(sum.as_uvec3());
        // }

        if
        // self.lod_level() == 0 &&
        sum.z < 0
            || sum.y < 0
            || sum.x < 0
            || sum.x >= chunk_size as i32
            || sum.y >= chunk_size as i32
            || sum.z >= chunk_size as i32
        {
            // TODO
            // for layer_idx in (0..4).rev() {
            //     let voxel_id = plat[layer_idx].get_node_gpu_no_enum(
            //         // ((self.position() * real_chunk_size).as_ivec3() + sum).as_uvec3(),
            //         // self.lod_level(),
            //         uvec3(0, 0, 0),
            //         0,
            //     );

            //     // if voxel_id != 0 {
            //     //     return voxel_id as u32;
            //     // }
            // }

            1
        } else {
            return self.get_unchecked(sum.as_uvec3());
        }
    }
}
