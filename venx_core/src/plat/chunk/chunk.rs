use core::mem::size_of;

use bytemuck::{Pod, Zeroable};
use bytes_cast::BytesCast;
use spirv_std::glam::{uvec3, UVec3};

use crate::utils::l2s;

#[repr(u8)]
// #[derive(Clone, Pod, Zeroable)]
pub enum Chunk {
    // X64+ = stackoverflow
    // X64(ChunkBase<64>),
    X32(ChunkBase<32>) = 0,
    X16(ChunkBase<16>) = 1,
    X8(ChunkBase<8>) = 2,
    X4(ChunkBase<4>) = 3,
}
// #[repr(transparent)]
// #[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkBase<const SIZE: usize> {
    pub mtx: [[[u32; SIZE]; SIZE]; SIZE], // TODO: flatten chunk
    pub position: UVec3,
    pub lod_level: u8,
    pub chunk_level: u8,
}

#[macro_export]
macro_rules! chunk {
    ($chunk:ident, $field:ident,$($ex:tt),+) => {
        match $chunk {
            // Chunk::X64(chunk) => chunk.$field$($ex)+,
            Chunk::X32(chunk) => chunk.$field$($ex)+,
            Chunk::X16(chunk) => chunk.$field$($ex)+,
            Chunk::X8(chunk) => chunk.$field$($ex)+,
            Chunk::X4(chunk) => chunk.$field$($ex)+,
        }
    };

    ($chunk:ident, $field:ident) => {
        match $chunk {
            // Chunk::X64(chunk) => chunk.$field,
            Chunk::X32(chunk) => chunk.$field,
            Chunk::X16(chunk) => chunk.$field,
            Chunk::X8(chunk) => chunk.$field,
            Chunk::X4(chunk) => chunk.$field,
        }
    };
}

impl<const SIZE: usize> ChunkBase<SIZE> {
    fn new(position: UVec3, lod_level: u8, chunk_level: u8) -> Self {
        // TODO: check size for cerrectnes with lod_level and chunk_level
        Self {
            mtx: [[[0; SIZE]; SIZE]; SIZE],
            position: position.into(),
            lod_level,
            chunk_level,
        }
    }
}
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub unsafe fn u8_slice_as_any<T: Sized>(p: &[u8]) -> &[T] {
    // ::core::slice::from_raw_parts((p as *const u8) as *const T, ::core::mem::size_of::<T>())
    todo!()
}

impl Chunk {
    pub fn lod_level(&self) -> u8 {
        chunk!(self, lod_level)
    }
    pub fn chunk_level(&self) -> u8 {
        chunk!(self, chunk_level)
    }
    pub fn position(&self) -> UVec3 {
        chunk!(self, position)
    }
    /// Size in blocks, tells how many blocks in chunk
    pub fn size(&self) -> u32 {
        match self {
            // Chunk::X64(chunk) => l2s(chunk.chunk_level - chunk.lod_level),
            Chunk::X32(chunk) => l2s(chunk.chunk_level - chunk.lod_level),
            Chunk::X16(chunk) => l2s(chunk.chunk_level - chunk.lod_level),
            Chunk::X8(chunk) => l2s(chunk.chunk_level - chunk.lod_level),
            Chunk::X4(chunk) => l2s(chunk.chunk_level - chunk.lod_level),
        }
    }
    /// Width in meters, tells how much space in 3d space chunk takes
    pub fn width(&self) -> u32 {
        l2s(chunk!(self, chunk_level))
    }
    pub fn level(&self) -> u8 {
        chunk!(self, chunk_level)
    }
    pub fn new(position: impl Into<UVec3>, lod_level: u8, chunk_level: u8) -> Self {
        let mtx_size = 1 << (chunk_level - lod_level);
        match mtx_size {
          //  64 => Chunk::X64(ChunkBase::new(position.into(), lod_level, chunk_level)),
                        32 => Chunk::X32(ChunkBase::new(position.into(), lod_level, chunk_level)),
                        16 => Chunk::X16(ChunkBase::new(position.into(), lod_level, chunk_level)),
                        8 => Chunk::X8(ChunkBase::new(position.into(), lod_level, chunk_level)),
                        4 => Chunk::X4(ChunkBase::new(position.into(), lod_level, chunk_level)),

            _ => panic!("Oh boy, I think you are trying to create a Chunk with unsupported size ({mtx_size})")
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
    pub fn get_raw(&self, block_position: impl Into<UVec3>) -> u32 {
        let pos = block_position.into();
        chunk!(
            self,
            mtx,
            [pos.x as usize],
            [pos.y as usize],
            [pos.z as usize]
        )
    }
    /// Sets local positioned block
    pub fn set(&mut self, position: impl Into<UVec3>, block: u32) {
        let position = position.into();
        chunk!(
            self,
            mtx,
            [position.x as usize],
            [position.y as usize],
            [position.z as usize],
            =,
            block
        );
        //self.mtx[position.x as usize][position.y as usize][position.z as usize] = block;
    }
    /// Iterating over local positions and blocks
    pub fn iter<F>(&self, mut callback: F)
    where
        F: FnMut(UVec3, u32),
    {
        // TODO: yeah, DRY is something i have never ever heard of.
        match self {
            // Chunk::X64(chunk) => {
            //     for (x, x_row) in chunk.mtx.iter().enumerate() {
            //         for (y, y_row) in x_row.iter().enumerate() {
            //             for (z, block) in y_row.iter().enumerate() {
            //                 callback(uvec3(x as u32, y as u32, z as u32), *block);
            //             }
            //         }
            //     }
            // }
            Chunk::X32(chunk) => {
                for (x, x_row) in chunk.mtx.iter().enumerate() {
                    for (y, y_row) in x_row.iter().enumerate() {
                        for (z, block) in y_row.iter().enumerate() {
                            callback(uvec3(x as u32, y as u32, z as u32), *block);
                        }
                    }
                }
            }
            Chunk::X16(chunk) => {
                for (x, x_row) in chunk.mtx.iter().enumerate() {
                    for (y, y_row) in x_row.iter().enumerate() {
                        for (z, block) in y_row.iter().enumerate() {
                            callback(uvec3(x as u32, y as u32, z as u32), *block);
                        }
                    }
                }
            }
            Chunk::X8(chunk) => {
                for (x, x_row) in chunk.mtx.iter().enumerate() {
                    for (y, y_row) in x_row.iter().enumerate() {
                        for (z, block) in y_row.iter().enumerate() {
                            callback(uvec3(x as u32, y as u32, z as u32), *block);
                        }
                    }
                }
            }
            Chunk::X4(chunk) => {
                for (x, x_row) in chunk.mtx.iter().enumerate() {
                    for (y, y_row) in x_row.iter().enumerate() {
                        for (z, block) in y_row.iter().enumerate() {
                            callback(uvec3(x as u32, y as u32, z as u32), *block);
                        }
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::Chunk;

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
