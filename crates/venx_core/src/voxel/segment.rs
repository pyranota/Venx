use glam::{uvec3, UVec3};

use super::cpu::utils::lvl_to_size::lvl_to_size;

pub struct Segment {
    pub level: u8,
    pub mtx: Vec<Vec<Vec<u32>>>,
}
pub struct SegmentStatic<const SIZE: usize> {
    pub mtx: [[[u32; SIZE]; SIZE]; SIZE],
}

impl Segment {
    pub fn size(&self) -> u32 {
        lvl_to_size(self.level)
    }
    pub fn level(&self) -> u8 {
        self.level
    }
    pub fn new(level: u8) -> Self {
        let mtx_size = 1 << level;
        Segment {
            mtx: vec![vec![vec![0; mtx_size]; mtx_size]; mtx_size],
            level,
        }
    }
    pub fn set(&mut self, position: impl Into<UVec3>, block: u32) {
        let position = position.into();
        self.mtx[position.x as usize][position.y as usize][position.z as usize] = block;
    }
    /// Iterate over local coordinates and its block types
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, u32),
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
        F: FnMut(UVec3, &mut u32),
    {
        for (x, x_row) in self.mtx.iter_mut().enumerate() {
            for (y, y_row) in x_row.iter_mut().enumerate() {
                for (z, block) in y_row.iter_mut().enumerate() {
                    callback(uvec3(x as u32, y as u32, z as u32), block);
                }
            }
        }
    }
}
