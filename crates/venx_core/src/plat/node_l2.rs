use core::fmt::Debug;

use spirv_std::glam::UVec3;

use super::node::{AllocatableNode, Node};

#[repr(transparent)]
#[derive(
    Copy, Clone, Default, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable, Hash, Eq,
)]
#[cfg_attr(feature = "bitcode_support", derive(bitcode::Encode, bitcode::Decode))]
/// Nodes on level_2
///
/// Basically each NodeL2 represents 4x4x4 grid of voxels
///
/// Why? It uses 4.5 times less memory than [Node]
pub struct NodeL2 {
    /// [[u8, u8, u8, u8], [u8, u8, u8, u8]]
    pub(crate) packed_children: [u32; 2],
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
    /// Get child on 1st level. index must be 0 to 7. Returns value between 0 and 256
    pub fn index_l1(&self, index: usize) -> u32 {
        let mut offset = index;
        let idx = offset / 4;
        offset %= 4;

        (self.packed_children[idx] >> (3 - offset) * 8) & 0b11_11_11_11
    }
    /// Get child on 0th level. index must be 0 to (8 * 8 - 1). Returns value between 0 and 1
    pub fn index_l0(&self, _index: usize) -> bool {
        todo!()
    }
    /// Unset voxel within `4x4x4` grid
    pub fn unset(&mut self, position: UVec3) {
        let (outer_offset, inner_offset, idx) = Self::get_offsets(position);
        self.packed_children[idx] &= !(1 << inner_offset << (outer_offset * 8));
    }
    #[inline]
    fn get_offsets(mut position: UVec3) -> (usize, usize, usize) {
        if position.x > 3 || position.y > 3 || position.z > 3 {
            panic!()
        }

        let mut outer_offset = Node::get_child_index(position, 1);
        let idx = outer_offset / 4;
        // TOPROFILE: Potential improvements possible
        outer_offset %= 4;
        outer_offset = 3 - outer_offset;
        let size = 2;
        position.x %= size;
        position.y %= size;
        position.z %= size;

        let inner_offset = Node::get_child_index(position, 0);

        (outer_offset, inner_offset, idx)
    }
}

impl AllocatableNode for NodeL2 {
    fn get_child(&self, index: usize) -> u32 {
        self.packed_children[index]
    }

    fn get_flag(&self) -> i32 {
        // Fool allocator, so it thinks its free node.
        // In reality NodeL2 has no flag, so it just not storing such information
        -1
    }

    fn set_child(&mut self, index: usize, child: u32) {
        self.packed_children[index] = child
    }

    fn set_flag(&mut self, _flag: i32) {
        // Ye we defenitely doing this work here. Lol
        // Btw flags will be removed at some point in future
        // Plus child in regular [Node] will take up 3 bytes instead of 4
    }

    fn get_first_free_link(layer: &super::layer::layer::Layer) -> u32 {
        layer.level_2[0].packed_children[0]
    }

    fn set_first_free_link(layer: &mut super::layer::layer::Layer, new_link: u32) {
        layer.level_2[0].packed_children[0] = new_link
    }

    fn get_node_mut<'a>(
        layer: &'a mut super::layer::layer::Layer<'_>,
        index: usize,
    ) -> &'a mut Self {
        &mut layer.level_2[index]
    }

    fn name() -> &'static str {
        "NodeLevel2"
    }
}

impl Debug for NodeL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NodeL2: ")?;
        let (children_left, children_right) = unsafe {
            ::core::slice::from_raw_parts(
                (&self.packed_children as *const [u32; 2]) as *const u8,
                ::core::mem::size_of::<[u32; 2]>(),
            )
        }
        .split_at(4);
        for child in children_left.iter().rev() {
            write!(f, "{:#010b} ", child)?;
        }
        for child in children_right.iter().rev() {
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

    use alloc::{borrow::ToOwned, format, vec::Vec};
    use rand::Rng;
    use spirv_std::glam::uvec3;

    use crate::{plat::node::NodeAddr, test_utils::gen_rand_mtx};

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
    fn test_position() {
        let mut n = NodeL2::default();

        let mut rng = rand::thread_rng();
        let p: (u32, u32, u32) = (
            rng.gen_range(0..4),
            rng.gen_range(0..4),
            rng.gen_range(0..4),
        );

        n.set(p.into());

        let addr = NodeAddr::from_position(p.into(), 3, 0);

        dbg!(addr.get_idx(2));
        dbg!(addr.get_idx(1));
        dbg!(addr.get_idx(0));

        dbg!(n);

        assert_eq!(n.index_l1(addr.get_idx(2)), 1 << addr.get_idx(1));
    }

    #[test]
    fn test_full() {
        let mut n = NodeL2::default();

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

    #[test]
    fn test_index_l1() {
        let mut n = NodeL2::default();

        let mtx = gen_rand_mtx::<4>(50);
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if mtx[x as usize][y as usize][z as usize] != 0 {
                        n.set(uvec3(x, y, z));
                    }
                }
            }
        }

        let st = format!("{n:?}");
        let parts: Vec<&str> = st.split_whitespace().skip(1).collect();

        assert_eq!(format!("{:#010b}", n.index_l1(0)), parts[0]);
        assert_eq!(format!("{:#010b}", n.index_l1(1)), parts[1]);
        assert_eq!(format!("{:#010b}", n.index_l1(2)), parts[2]);
        assert_eq!(format!("{:#010b}", n.index_l1(3)), parts[3]);
        assert_eq!(format!("{:#010b}", n.index_l1(4)), parts[4]);
        assert_eq!(format!("{:#010b}", n.index_l1(5)), parts[5]);
        assert_eq!(format!("{:#010b}", n.index_l1(6)), parts[6]);
        assert_eq!(format!("{:#010b}", n.index_l1(7)), parts[7]);
    }
}
