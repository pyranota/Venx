use glam::UVec3;

use crate::{chunk::chunk::Chunk, voxel::cpu::utils};

use super::voxel::Voxel;

impl Voxel {
    pub fn load_chunk(&self, position: UVec3, lod_level: u8, chunk_level: u8) -> Chunk {
        let mtx_size = 1 << (chunk_level - lod_level);
        let mut chunk = Chunk {
            mtx: vec![vec![vec![false; mtx_size]; mtx_size]; mtx_size],
            position,
            lod_level,
        };

        let chunk_size = utils::lvl_to_size::lvl_to_size(chunk_level);
        todo!()
    }
}
