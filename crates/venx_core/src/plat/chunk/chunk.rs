use bytemuck::{Pod, Zeroable};
use bytes_cast::BytesCast;
use spirv_std::{
    glam::{uvec3, UVec3},
    num_traits::Zero,
};

use crate::utils::l2s;

// #[cfg(not(feature = "bitcode_support"))]
// const MAX_SIZE: usize = 32;

// #[cfg(feature = "bitcode_support")]
const MAX_SIZE: usize = 32 * 32 * 32;

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
    pub(crate) data: [u32; MAX_SIZE + 6],
    // position: UVec3,
    // lod_level: usize,
    // chunk_level: usize,
    // size: usize,
    // TODO:
    // neighbor chunk levels
}

unsafe impl Pod for Chunk {}

unsafe impl Zeroable for Chunk {}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct ChunkMeta {
    x: usize,
    y: usize,
    z: usize,
    lod_level: usize,
    chunk_level: usize,
    size: usize,
    // TODO:
    // neighbor chunk levels
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub unsafe fn u8_slice_as_any<T: Sized>(p: &[u8]) -> &[T] {
    // ::core::slice::from_raw_parts((p as *const u8) as *const T, ::core::mem::size_of::<T>())

    todo!()
}

impl Chunk {
    pub fn lod_level(&self) -> usize {
        self.data[3] as usize
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

    // pub fn to_send(self) -> ([u32; MAX_SIZE], ChunkMeta) {
    //     (
    //         self.flatten,
    //         ChunkMeta {
    //             x: self.position.x as usize,
    //             y: self.position.y as usize,
    //             z: self.position.z as usize,
    //             lod_level: self.lod_level,
    //             chunk_level: self.chunk_level,
    //             size: self.size,
    //         },
    //     )
    // }

    // pub fn receive(flatten: [u32; MAX_SIZE], meta: ChunkMeta) -> Self {
    //     Self {
    //         flatten,
    //         position: uvec3(meta.x as u32, meta.y as u32, meta.z as u32),
    //         lod_level: meta.lod_level,
    //         chunk_level: meta.chunk_level,
    //         size: meta.size,
    //     }
    // }

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
    pub fn get(&self, block_position: UVec3) -> Option<u32> {
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
        let idx = self.flatten_value(position);

        self.data[idx] = block;
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
    /// Warning! In chunk you have global position, in segment local
    pub fn iter_mut<F>(&mut self, mut callback: F)
    where
        F: FnMut(UVec3, &mut u32),
    {
        todo!("Is anybody even using it?")
        // let size = self.size();
        // for (x, x_row) in self.mtx.iter_mut().enumerate() {
        //     for (y, y_row) in x_row.iter_mut().enumerate() {
        //         for (z, block) in y_row.iter_mut().enumerate() {
        //             callback(
        //                 uvec3(x as u32, y as u32, z as u32) + (self.position * size),
        //                 block,
        //             );
        //         }
        //     }
        // }
    }
}
#[cfg(feature = "bitcode_support")]
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
        let chunk = Chunk::new((0, 0, 0), 0, 2);
        // 8
        let chunk = Chunk::new((0, 0, 0), 0, 3);
        // 16
        let chunk = Chunk::new((0, 0, 0), 0, 4);
        // 32
        let chunk = Chunk::new((0, 0, 0), 0, 5);
        // // 64
        // let chunk = Chunk::new((0, 0, 0), 0, 6);
    }
}
