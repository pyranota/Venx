use core::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::UVec3;

use crate::utils::l2s;

use super::layer::layer::Layer;

#[repr(C)]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Pod, Zeroable)]
#[cfg_attr(feature = "bitcode_support", derive(bitcode::Encode, bitcode::Decode))]
pub struct Node {
    /// ` 0` - branch
    ///
    /// `-1` - free node
    ///
    /// `-3` - leaf
    ///
    /// `-4` - compact fork with 8 subtrees
    ///
    /// `-5` - fork with 4 subtrees and its ids
    ///
    /// `-6` - extended fork with atleast 1 fork
    ///
    /// `0 < n` - subtree with voxel-id
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
    // TODO: move to Layer
    /// Allows to work without even thinking about forks
    /// Usefull if node points to fork, but this abstracts it, and just returns idx to node below this all
    pub fn set_child(
        layer: &mut Layer,
        from_idx: usize,
        voxel_id: u32,
        child_idx: usize,
        level: usize,
        fork_level: usize,
        // TODO:
        // force_extended: bool
    ) -> usize {
        let mut idx = from_idx;
        let node = &mut layer[from_idx];

        // If children of node on from_idx should point to forks
        if level == fork_level + 1 {
            let child_node_idx = node.children[child_idx];

            if child_node_idx == 0 {
                // Fork does not exist, lets change that
                let new_node_idx = layer.allocate_node();

                layer[new_node_idx].flag = voxel_id as i32;

                let fork = Node {
                    flag: -5,
                    children: [voxel_id, new_node_idx as u32, 0, 0, 0, 0, 0, 0],
                };

                layer.allocate_node_from(fork);

                return new_node_idx;
            } else {
                let fork = &mut layer[child_node_idx as usize];

                // Best variant
                if fork.flag == -5 {
                    // Left to find needed link to subtree with given voxel_id
                    // TODO: Try to use iter().enumerate().step(2) or similar
                    for voxel_id_idx in [0, 2, 4, 6] {
                        let found_voxel_id = fork[voxel_id_idx];

                        if found_voxel_id == voxel_id {
                            // Bravo, we found idx to forward!
                            // We had flag -5, thats why we can do it so simply
                            return fork[voxel_id_idx + 1] as usize;
                        }
                    }
                    // Whoopsie, looks like we need to rearrange everything a bit
                    // We are out of slots for voxel_id's
                    // Now we need to move from normal fork to compact one
                    // It might be slow, but its not very often situation in normal use cases
                    // During generation used extended forks with 8 normal forks
                    // And after they are shrinked (like merge)

                    // Make sure our fork is treated as compact one
                    fork.flag = -4;

                    for node_idx_idx in [1, 3, 5, 7] {
                        let node_idx = fork[node_idx_idx];
                        // Shifting node_idx
                        fork[(node_idx_idx - 1) / 2] = node_idx;
                        // Clean up if we are above the center
                        // All nodes before are getting overwritten by each other, so no need to clean them up
                        if node_idx_idx >= 4 {
                            fork[node_idx_idx] = 0;
                        }
                    }

                    // And now used is just the half (4) and the rest is empty
                    // So we can allocate our new node to voxel_id
                    let new_node_idx = layer.allocate_node();
                    // Set fork 5th child to new node
                    layer[child_node_idx as usize][4] = new_node_idx as u32;

                    let new_node = &mut layer[new_node_idx];
                    // Set voxel_id, so it can be readed
                    new_node.flag = voxel_id as i32;

                    return new_node_idx;
                }
                // Not the worse case
                else if fork.flag == -4 {
                    for node_idx_idx in 0..8 {
                        let found_node_idx = layer[child_node_idx as usize][node_idx_idx];

                        let fork = &mut layer[found_node_idx as usize];
                        if fork.flag == voxel_id as i32 {
                            // Bravo, we found idx to forward!
                            return found_node_idx as usize;
                        }
                        // Well, no voxel_id in there
                        // Lets create one
                        // But there is no place left, lets rearrange it once again
                        // Now from compact node to extended with first child as link to compact fork and the rest for just regular nodes
                        // Clone this fork to add as first child
                        let sub_fork = fork.clone();

                        let new_fork_idx = layer.allocate_node_from(sub_fork);

                        let fork = &mut layer[found_node_idx as usize];

                        fork.flag = -6;
                        fork.children = [0; 8];
                        fork[0] = new_fork_idx as u32;
                    }
                }
                // The worse and the slowest scenario
                else if fork.flag == -6 {
                    for node_idx_idx in 0..8 {
                        loop {
                            let found_node_idx = layer[child_node_idx as usize][node_idx_idx];

                            let fork = &mut layer[found_node_idx as usize];
                            let flag = fork.flag;
                            if flag == voxel_id as i32 {
                                // Bravo, we found idx to forward!
                                return found_node_idx as usize;
                            } else if flag == -4 {
                            }

                            // Implement with EStack
                        }
                        // // Well, no voxel_id in there
                        // // Lets create one
                        // // But there is no place left, lets rearrange it once again
                        // // Now from compact node to extended with first child as link to compact fork and the rest for just regular nodes
                        // // Clone this fork to add as first child
                        // let sub_fork = fork.clone();

                        // let new_fork_idx = layer.allocate_node_from(sub_fork);

                        // let fork = &mut layer[found_node_idx as usize];

                        // fork.flag = -6;
                        // fork.children = [0; 8];
                        // fork[0] = new_fork_idx as u32;
                    }
                    todo!("Support for extended forks is not yet implemented")
                }
            }
        }

        // if node.flag == -5 {
        //     for i in [0, 2, 4, 6] {
        //         let found_voxel_id = node.children[i];

        //         if found_voxel_id == voxel_id as u32 {
        //             // Use if there is already
        //             idx = node.children[i - 1] as usize;
        //             break;
        //         } else if found_voxel_id == 0 {
        //             // Allocated new if there is no
        //         }
        //     }
        // }
        todo!()
    }
}

impl Index<usize> for Node {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index_mut: usize) -> &mut Self::Output {
        &mut self.children[index_mut]
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
