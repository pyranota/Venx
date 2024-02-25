use core::fmt::Debug;

use spirv_std::glam::UVec3;

use super::node::Node;

#[repr(transparent)]
#[derive(
    Copy, Clone, Default, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable, Hash, Eq,
)]
#[cfg_attr(feature = "bitcode_support", derive(bitcode::Encode, bitcode::Decode))]
/// Nodes on level_2
///
/// Basically each NodeL2 represents 4x4x4 grid of voxels
pub struct NodeL2 {
    /// [[u8, u8, u8, u8], [u8, u8, u8, u8]]
    packed_children: [u32; 2],
}
// Question?
// What if different compute shader invocations will write at the same time, but different positions?
// Will it corrupt data?
// Or it will be alright?
// I think since we have `|=` (or) binary operator it will be ok. But need to test
impl NodeL2 {
    /// Set voxel within `4x4x4` grid
    pub fn set(&mut self, position: UVec3) {
        let (outer_offset, inner_offset, idx) = Self::get_offsets(position);
        self.packed_children[idx] |= 1 << inner_offset << (outer_offset * 8);
    }
    /// Check if there is a voxel within given `4x4x4` grid
    pub fn is_at(&self, position: UVec3) -> bool {
        let (outer_offset, inner_offset, idx) = Self::get_offsets(position);
        self.packed_children[idx] & 1 << inner_offset << (outer_offset * 8) != 0
    }
    /// Unset voxel within `4x4x4` grid
    pub fn unset(&mut self, position: UVec3) {
        let (outer_offset, inner_offset, idx) = Self::get_offsets(position);
        self.packed_children[idx] &= !(1 << inner_offset << (outer_offset * 8));
    }

    fn get_offsets(mut position: UVec3) -> (usize, usize, usize) {
        // TODO: Do bound checking
        let mut outer_offset = Node::get_child_index(position, 1);
        let idx = outer_offset / 4;
        // TOPROFILE: Potential improvements possible
        outer_offset %= 4;

        let size = 2;
        position.x %= size;
        position.y %= size;
        position.z %= size;

        let inner_offset = Node::get_child_index(position, 0);

        (outer_offset, inner_offset, idx)
    }
}

impl Debug for NodeL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NodeL2: ")?;
        let children = unsafe {
            ::core::slice::from_raw_parts(
                (&self.packed_children as *const [u32; 2]) as *const u8,
                ::core::mem::size_of::<[u32; 2]>(),
            )
        };
        for child in children {
            write!(f, "{:#010b} ", child)?;
        }
        write!(f, "\n")
    }
}

#[cfg(feature = "bitcode_support")]
#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;
    use std::dbg;

    use alloc::{borrow::ToOwned, format};
    use spirv_std::glam::uvec3;

    use crate::test_utils::gen_rand_mtx;

    use super::NodeL2;

    #[test]
    fn test_simple_get_set() {
        let mut n = NodeL2::default();
        // 0 0 0
        n.set((0, 0, 0).into());
        // 0 1 0
        n.set((0, 2, 0).into());
        // 0 0 1
        n.set((0, 0, 2).into());
        // 1 0 0
        n.set((2, 0, 0).into());
        // 1 0 1
        n.set((2, 0, 2).into());
        // 1 1 1
        n.set((2, 2, 2).into());
        // 1 1 0
        n.set((2, 2, 0).into());
        // 0 1 1
        n.set((0, 2, 2).into());

        assert_eq!(format!("{n:?}"), "NodeL2: 0b00000001 0b00000001 0b00000001 0b00000001 0b00000001 0b00000001 0b00000001 0b00000001 \n".to_owned());
        assert!(n.is_at((0, 0, 0).into()));
        assert!(n.is_at((0, 2, 0).into()));
        assert!(n.is_at((0, 2, 2).into()));
        assert!(n.is_at((2, 2, 0).into()));
        assert!(n.is_at((2, 2, 2).into()));
        assert!(n.is_at((2, 0, 0).into()));
    }

    #[test]
    fn test_simple_unset() {
        let mut n = NodeL2::default();

        n.set((0, 0, 0).into());
        n.set((0, 1, 0).into());
        n.set((1, 1, 0).into());
        n.set((0, 1, 1).into());
        n.set((1, 1, 1).into());

        let b = n.clone();

        n.set((1, 0, 1).into());
        n.unset((1, 0, 1).into());

        assert_eq!(b, n);
    }

    #[test]
    fn test_full() {
        let mut n = NodeL2::default();

        n.set((0, 0, 0).into());
        n.set((0, 2, 0).into());

        let mtx = gen_rand_mtx::<4>(50);
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if mtx[x as usize][y as usize][z as usize] != 0 {
                        n.set(uvec3(x, y, z));
                        assert!(n.is_at(uvec3(x, y, z)));
                    }
                }
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if mtx[x as usize][y as usize][z as usize] != 0 {
                        assert!(n.is_at(uvec3(x, y, z)));
                    }
                }
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if mtx[x as usize][y as usize][z as usize] != 0 {
                        n.unset(uvec3(x, y, z));
                        assert!(!n.is_at(uvec3(x, y, z)));
                    }
                }
            }
        }

        assert_eq!(n, NodeL2::default());
    }
}
