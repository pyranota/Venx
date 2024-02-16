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
#[repr(C)]
pub struct Chunk {
    pub flatten: [u32; MAX_SIZE],
    position: UVec3,
    lod_level: usize,
    chunk_level: usize,
    size: usize,
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
        self.lod_level
    }
    pub fn chunk_level(&self) -> usize {
        self.chunk_level
    }
    pub fn position(&self) -> UVec3 {
        self.position
    }
    /// Size in blocks, tells how many blocks in chunk
    pub fn size(&self) -> u32 {
        self.size as u32
    }
    /// Width in meters, tells how much space in 3d space chunk takes
    pub fn width(&self) -> u32 {
        l2s(self.chunk_level)
    }

    pub fn to_send(self) -> ([u32; MAX_SIZE], ChunkMeta) {
        (
            self.flatten,
            ChunkMeta {
                x: self.position.x as usize,
                y: self.position.y as usize,
                z: self.position.z as usize,
                lod_level: self.lod_level,
                chunk_level: self.chunk_level,
                size: self.size,
            },
        )
    }

    pub fn receive(flatten: [u32; MAX_SIZE], meta: ChunkMeta) -> Self {
        Self {
            flatten,
            position: uvec3(meta.x as u32, meta.y as u32, meta.z as u32),
            lod_level: meta.lod_level,
            chunk_level: meta.chunk_level,
            size: meta.size,
        }
    }

    pub fn new(position: impl Into<UVec3>, lod_level: usize, chunk_level: usize) -> Self {
        // let mtx_size = 1 << (chunk_level - lod_level);
        Self {
            flatten: [0; MAX_SIZE],
            position: position.into(),
            lod_level,
            chunk_level,
            size: l2s(chunk_level - lod_level) as usize,
        }
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
        let width = self.width();
        (p.x + (p.y * width) + (p.z * width * width)) as usize
    }

    // Index in chunk to 3D Position
    pub fn from_flatten(&self, mut index: u32) -> UVec3 {
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
        self.flatten[idx]
    }
    /// Sets local positioned block
    pub fn set(&mut self, position: UVec3, block: u32) {
        let idx = self.flatten_value(position);

        self.flatten[idx] = block;
    }
    /// Iterating over local positions and blocks
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, u32),
    {
        let size = self.size;
        for index in 0..(size * size * size) {
            let voxel_id = self.flatten[index];

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
    use super::Chunk;

    #[test]
    fn test_chunk_iter() {
        let mut chunk = Chunk::new((0, 0, 0), 0, 4);
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
