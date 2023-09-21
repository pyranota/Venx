use glam::{uvec3, UVec3};

#[derive(Debug)]
pub struct Chunk {
    pub mtx: Vec<Vec<Vec<bool>>>,
    pub position: UVec3,
    pub lod_level: u8,
    pub chunk_level: u8,
}
impl Chunk {
    pub fn new(position: UVec3, lod_level: u8, chunk_level: u8) -> Self {
        let mtx_size = 1 << (chunk_level - lod_level);
        Chunk {
            mtx: vec![vec![vec![false; mtx_size]; mtx_size]; mtx_size],
            position,
            lod_level,
            chunk_level,
        }
    }
    pub fn set(&mut self, position: UVec3, block: bool) {
        self.mtx[position.x as usize][position.y as usize][position.z as usize] = block;
    }
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, bool),
    {
        for (x, x_row) in self.mtx.iter().enumerate() {
            for (y, y_row) in x_row.iter().enumerate() {
                for (z, block) in y_row.iter().enumerate() {
                    callback(uvec3(x as u32, y as u32, z as u32) + self.position, *block);
                }
            }
        }
    }
    pub fn iter_mut<F>(&mut self, mut callback: F)
    where
        F: FnMut(UVec3, &mut bool),
    {
        for (x, x_row) in self.mtx.iter_mut().enumerate() {
            for (y, y_row) in x_row.iter_mut().enumerate() {
                for (z, block) in y_row.iter_mut().enumerate() {
                    callback(uvec3(x as u32, y as u32, z as u32) + self.position, block);
                }
            }
        }
    }
}
#[test]
fn test_chunk_iter() {
    let mut chunk = Chunk::new(UVec3::ZERO, 0, 4);
    chunk.set(uvec3(4, 4, 0), true);

    chunk.iter(|pos, block| {
        if block {
            assert_eq!(pos, uvec3(4, 4, 0));
        }
    });
    chunk.iter_mut(|pos, block| {
        if *block {
            *block = false;
        }
    });
    chunk.iter(|pos, block| {
        if block {
            panic!();
        }
    });
}
