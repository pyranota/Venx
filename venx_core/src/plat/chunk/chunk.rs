use glam::{uvec3, UVec3};

use crate::voxel::cpu::utils::lvl_to_size::lvl_to_size;

#[derive(Debug)]
pub struct Chunk {
    pub mtx: &mut [u32], // flatten chunk
    pub position: UVec3,
    pub lod_level: u8,
    pub chunk_level: u8,
}
impl Chunk {
    /// Size in blocks, tells how many blocks in chunk
    pub fn size(&self) -> u32 {
        lvl_to_size(self.chunk_level - self.lod_level)
    }
    /// Width in meters, tells how much space in 3d space chunk takes
    pub fn width(&self) -> u32 {
        lvl_to_size(self.chunk_level)
    }
    pub fn level(&self) -> u8 {
        self.chunk_level
    }
    pub fn new(position: impl Into<UVec3>, lod_level: u8, chunk_level: u8) -> Self {
        let mtx_size = 1 << (chunk_level - lod_level);
        Chunk {
            mtx: vec![vec![vec![0; mtx_size]; mtx_size]; mtx_size],
            position: position.into(),
            lod_level,
            chunk_level,
        }
    }
    pub fn get(&self, block_position: UVec3) -> Option<i32> {
        // Check for out of bound
        if block_position.max_element() >= self.size() {
            return None;
        }

        let val = self.get_raw(block_position);
        if val != 0 {
            return Some(val);
        } else {
            return None;
        }
    }
    pub fn get_raw(&self, block_position: impl Into<UVec3>) -> i32 {
        let pos = block_position.into();

        self.mtx[pos.x as usize][pos.y as usize][pos.z as usize]
    }
    /// Sets local positioned block
    pub fn set(&mut self, position: impl Into<UVec3>, block: i32) {
        let position = position.into();
        self.mtx[position.x as usize][position.y as usize][position.z as usize] = block;
    }
    /// Iterating over local positions and blocks
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, i32),
    {
        for (x, x_row) in self.mtx.iter().enumerate() {
            for (y, y_row) in x_row.iter().enumerate() {
                for (z, block) in y_row.iter().enumerate() {
                    callback(uvec3(x as u32, y as u32, z as u32), *block);
                }
            }
        }
    }
    /// Warning! In chunk you have global position, in segment local
    pub fn iter_mut<F>(&mut self, mut callback: F)
    where
        F: FnMut(UVec3, &mut i32),
    {
        let size = self.size();
        for (x, x_row) in self.mtx.iter_mut().enumerate() {
            for (y, y_row) in x_row.iter_mut().enumerate() {
                for (z, block) in y_row.iter_mut().enumerate() {
                    callback(
                        uvec3(x as u32, y as u32, z as u32) + (self.position * size),
                        block,
                    );
                }
            }
        }
    }
}
#[test]
fn test_chunk_iter() {
    let mut chunk = Chunk::new((0, 0, 0), 0, 4);
    chunk.set((4, 4, 0), 4);

    chunk.iter(|pos, block| {
        if block != 0 {
            assert_eq!(pos, (4, 4, 0).into());
        }
    });
    chunk.iter_mut(|pos, block| {
        if *block != 0 {
            *block = 4;
        }
    });
    chunk.iter(|pos, block| {
        if block != 0 {
            panic!();
        }
    });
}
