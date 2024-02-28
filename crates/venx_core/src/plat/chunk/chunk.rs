use bytemuck::{Pod, Zeroable};

use spirv_std::glam::{uvec3, UVec3};

use crate::utils::l2s;

// #[cfg(not(feature = "bitcode_support"))]
// const MAX_SIZE: usize = 32;
const WIDTH: usize = 32;
// #[cfg(feature = "bitcode_support")]
const MAX_SIZE: usize = WIDTH * WIDTH * WIDTH;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Chunk {
    /// 0 - x
    ///
    /// 1 - y
    ///
    /// 2 - z
    ///
    /// 3 - lod_level
    ///
    /// 4 - chunk_level
    ///
    /// 5 - size
    ///
    /// Rest - flatten chunk
    pub data: [u32; MAX_SIZE + 6],
    // position: UVec3,
    // lod_level: usize,
    // chunk_level: usize,
    // size: usize,
    // TODO:
    // neighbor chunk levels
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            data: [0; MAX_SIZE + 6],
        }
    }
}

unsafe impl Pod for Chunk {}

unsafe impl Zeroable for Chunk {}

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct ChunkLoadRequest {
    pub position: [u32; 3],
    pub lod_level: u32,
    pub chunk_level: u32,
}

unsafe impl Pod for ChunkLoadRequest {}

unsafe impl Zeroable for ChunkLoadRequest {}

impl Chunk {
    pub fn lod_level(&self) -> usize {
        self.data[3] as usize
    }

    pub fn clean(&mut self) {
        for i in 0..MAX_SIZE {
            self.data[i + 6] = 0;
        }
    }

    pub fn blank_with(&mut self, d: u32) {
        for i in 0..MAX_SIZE {
            self.data[i + 6] = d;
        }
    }
    pub fn fill_layer(&mut self, layer: [u32; WIDTH * WIDTH]) {
        for i in 0..layer.len() {
            self.data[i + 6] = layer[i];
        }
    }
    pub fn chunk_level(&self) -> usize {
        self.data[4] as usize
    }
    pub fn position(&self) -> UVec3 {
        uvec3(self.data[0], self.data[1], self.data[2])
    }
    /// Size in blocks, tells how many blocks in chunk
    pub fn size(&self) -> u32 {
        self.data[5]
    }
    /// Width in meters, tells how much space in 3d space chunk takes
    pub fn width(&self) -> u32 {
        l2s(self.chunk_level())
    }

    pub fn new(position: impl Into<UVec3>, lod_level: usize, chunk_level: usize) -> Self {
        let mut data = [0; MAX_SIZE + 6];
        let p = position.into();
        data[0] = p.x;
        data[1] = p.y;
        data[2] = p.z;
        data[3] = lod_level as u32;
        data[4] = chunk_level as u32;
        data[5] = l2s(chunk_level - lod_level);
        Self { data }
    }

    pub fn update_meta(&mut self, p: UVec3, lod_level: usize, chunk_level: usize) {
        self.data[0] = p.x;
        self.data[1] = p.y;
        self.data[2] = p.z;
        self.data[3] = lod_level as u32;
        self.data[4] = chunk_level as u32;
        self.data[5] = l2s(chunk_level - lod_level);
    }
    pub fn get(&self, block_position: UVec3) -> Option<u32> {
        // Check for out of bound
        if block_position.x >= self.size()
            || block_position.y >= self.size()
            || block_position.z >= self.size()
        {
            return None;
        }

        let val = self.get_raw(block_position);
        if val != 0 {
            return Some(val);
        } else {
            return None;
        }
    }

    pub fn get_global(&self, mut block_position: UVec3) -> Option<u32> {
        block_position -= self.position() * self.width() as u32;
        // TODO: Potential bug                   ^^^^^^^
        self.get(block_position)
    }

    pub fn get_unchecked(&self, block_position: UVec3) -> u32 {
        // Check for out of bound
        if block_position.x >= self.size()
            || block_position.y >= self.size()
            || block_position.z >= self.size()
        {
            return 0;
        }

        self.get_raw(block_position)
    }

    /// 3D Position to index in chunk
    pub fn flatten_value(&self, p: UVec3) -> usize {
        let size = self.size();
        (p.x + (p.y * size) + (p.z * size * size)) as usize + 6
    }

    // Index in chunk to 3D Position
    pub fn from_flatten(&self, mut index: u32) -> UVec3 {
        index -= 6;
        let size = self.size();
        //assert!(0 <= index < size*size*size);
        let x = index % size;
        index /= size;
        let y = index % size;
        index /= size;
        let z = index;
        uvec3(x, y, z)
    }

    pub fn get_raw(&self, p: UVec3) -> u32 {
        let idx = self.flatten_value(p);
        self.data[idx]
    }
    /// Sets local positioned block
    pub fn set(&mut self, position: UVec3, block: u32) {
        assert!(position.x < self.size());
        assert!(position.y < self.size());
        assert!(position.z < self.size());

        let idx = self.flatten_value(position);

        self.data[idx] = block;
    }

    /// Sets global positioned block
    pub fn set_global(&mut self, mut position: UVec3, block: u32) {
        position -= self.position() * self.width() as u32;
        // TODO: Potential bug            ^^^^^^^
        self.set(position, block);
    }

    /// Iterating over local positions and blocks
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, u32),
    {
        let size = self.size();
        for index in 6..(size * size * size + 6) {
            let voxel_id = self.data[index as usize];

            if voxel_id != 0 {
                callback(self.from_flatten(index as u32), voxel_id);
            }
        }
    }
}
#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use rand::Rng;
    use spirv_std::glam::uvec3;

    use super::Chunk;

    #[test]
    fn test_chunk_iter() {
        let mut chunk = Chunk::new((0, 0, 0), 0, 4);
        chunk.set((4, 4, 0).into(), 44);

        chunk.iter(|pos, block| {
            assert_eq!(pos, (4, 4, 0).into());
            assert_eq!(block, 44);
        });
    }

    #[test]
    fn test_chunk_get() {
        let mut chunk = Chunk::new((0, 0, 0), 0, 4);
        chunk.set((4, 4, 0).into(), 144);
        chunk.set((1, 5, 1).into(), 434);
        chunk.set((2, 6, 2).into(), 4454);
        chunk.set((3, 7, 2).into(), 1414);

        assert_eq!(chunk.get((4, 4, 0).into()).unwrap(), 144);
        assert_eq!(chunk.get((1, 5, 1).into()).unwrap(), 434);
        assert_eq!(chunk.get((2, 6, 2).into()).unwrap(), 4454);
        assert_eq!(chunk.get((3, 7, 2).into()).unwrap(), 1414);
    }

    #[test]
    fn test_chunk_iter_voxel_id_full() {
        let mut chunk = Chunk::new((0, 0, 0), 0, 4);

        let mut rng = rand::thread_rng();

        let mtx: [[[u32; 16]; 16]; 16] = rng.gen();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    chunk.set(uvec3(x, y, z), mtx[x as usize][y as usize][z as usize]);
                }
            }
        }

        chunk.iter(|pos, block| {
            assert_eq!(chunk.get(pos).unwrap(), block);
        });
    }

    #[test]
    fn test_chunk_iter_lod() {
        let mut chunk = Chunk::new((0, 0, 0), 1, 5);
        chunk.set((4, 4, 0).into(), 4);

        chunk.iter(|pos, block| {
            assert_eq!(pos, (4, 4, 0).into());
            assert_eq!(block, 4);
        });
    }

    #[test]
    fn check_stack_overflow_limit() {
        // 4
        Chunk::new((0, 0, 0), 0, 2);
        // 8
        Chunk::new((0, 0, 0), 0, 3);
        // 16
        Chunk::new((0, 0, 0), 0, 4);
        // 32
        Chunk::new((0, 0, 0), 0, 5);
        // // 64
        // let chunk = Chunk::new((0, 0, 0), 0, 6);
    }
}
