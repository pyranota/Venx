use core::{fmt::Debug, ops::Index};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::UVec3;

use crate::utils::l2s;

#[repr(C)]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Pod, Zeroable)]
#[cfg_attr(feature = "bitcode_support", derive(bitcode::Encode, bitcode::Decode))]
pub struct Node {
    /// `0` - normal branch,
    /// `1` - link to shared,
    /// `-1` - empty node,
    /// `2` - tmp link to node on same level.
    /// `3` - leaf node (single node at 0 level with idx 1)
    /// `9` - not usable reserved node.
    /// if its `-1`, first child will be interpreted as link to the next empty node
    /// And second child as previous node. If there is no nodes it will be `0`
    pub flag: i32,
    // After reading this, you might be wondering,
    // Why not to use enums or any other data type
    // Thats why the same alghorithm should be implemented on cpu and gpu.
    // Howewer on gpu there is no enums and that fancy data structs
    // And for making things uniform and similar it uses that simple types.
    // Plus its makes much easier to convert and send to gpu.
    pub children: [u32; 8],
}

impl Node {
    /// Internal index of node converted to normalized vector
    pub fn get_child_position(i: u32) -> UVec3 {
        UVec3::new(i & 1, (i >> 1) & 1, (i >> 2) & 1)
    }
    /// Convert position of node in 3d space coordinate to internal child branch index
    pub fn get_child_index(pos: UVec3, level: usize) -> usize {
        let child_size = 1 << level;
        let x = if pos.x < child_size { 0 } else { 1 };
        let y = if pos.y < child_size { 0 } else { 1 };
        let z = if pos.z < child_size { 0 } else { 1 };
        (x + y * 2 + z * 4) as usize
    }
    pub fn new(level: usize, flag: i32) -> Self {
        Self {
            flag,
            children: Default::default(),
        }
    }
}
// TODO: implement From<u64>
pub struct NodeAddr(u64);

impl NodeAddr {
    pub fn from_position(mut position: UVec3, depth: usize) -> Self {
        assert!(depth <= 21);
        let mut addr = Self::new();
        let mut level = depth;
        let mut size = l2s(depth);
        while level > 0 {
            {
                let child_index = Node::get_child_index(position, level - 1);
                addr.set_idx(level, child_index);
            }

            {
                size /= 2;
                level -= 1;
                position.x %= size;
                position.y %= size;
                position.z %= size;
            }
        }

        addr
    }
    pub(crate) fn new() -> Self {
        NodeAddr(0)
    }
    /// Get child idx on given level
    pub fn get_idx(&self, level: usize) -> usize {
        assert!(level <= 21);
        ((&self.0 >> (level * 3)) & 0o7u64) as usize
    }
    /// Set child idx on given level
    /// Idx is 0-7
    pub fn set_idx(&mut self, level: usize, idx: usize) {
        assert!(level <= 21);
        assert!(idx < 8);
        self.0 |= (idx as u64) << level * 3;
    }
}

#[cfg(test)]
mod tests {
    use spirv_std::glam::uvec3;

    use super::NodeAddr;

    #[test]
    fn test_node_addr() {
        let mut addr = NodeAddr::new();
        addr.set_idx(5, 0);
        addr.set_idx(4, 5);
        addr.set_idx(3, 7);
        addr.set_idx(2, 1);
        addr.set_idx(1, 5);
        addr.set_idx(0, 6);

        assert_eq!(addr.get_idx(5), 0);
        assert_eq!(addr.get_idx(4), 5);
        assert_eq!(addr.get_idx(3), 7);
        assert_eq!(addr.get_idx(2), 1);
        assert_eq!(addr.get_idx(1), 5);
        assert_eq!(addr.get_idx(0), 6);
    }

    #[test]
    fn test_node_addr_from_position() {
        let addr = NodeAddr::from_position(uvec3(7, 20, 5), 5);

        assert_eq!(addr.get_idx(5), 2);
        assert_eq!(addr.get_idx(4), 0);
        assert_eq!(addr.get_idx(3), 7);
        assert_eq!(addr.get_idx(2), 1);
        assert_eq!(addr.get_idx(1), 5);
        assert_eq!(addr.get_idx(0), 0);
    }
}
