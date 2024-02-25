use core::ops::{Index, IndexMut};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{
        node::{Node, NodeAddr},
        node_l2::NodeL2,
        op::{get::GetNodeResult, traverse::Props},
        stack::EStack,
    },
    utils::l2s,
};

/// Alias for layer
pub type Lr<'a> = Layer<'a>;

#[derive(PartialEq)]
pub struct Layer<'a> {
    /// Synced depth with RawPlat
    pub depth: usize,

    /// On level 0 we have single node
    ///
    /// On level 1 we have 256 nodes
    ///
    /// On level 2 we have majority of nodes in entire layer
    ///
    /// But we dont need to have u32 to link each child, since on level 1 only 256 nodes
    ///
    /// Instead we have u8 to link each child.
    ///
    /// There is no level 0 or 1. They are "phantom" levels, which is "packed" in this u8 value
    ///
    /// Can be called `l2`
    ///
    /// [0] is head of `free nodes`
    pub level_2: &'a mut [NodeL2],

    /// [0] is head of `free nodes`
    ///
    /// [1] root
    pub nodes: &'a mut [Node],
}

pub struct ForkIterProps {
    pub drop: bool,
    pub voxel_id: usize,
    pub node_idx: usize,
}

impl<'a> Layer<'a> {
    /// Index to Base layer
    pub const BASE: usize = 0;
    /// Index to Tmp layer
    pub const TMP: usize = 1;
    /// Index to Schem layer
    pub const SCHEM: usize = 2;
    /// Index to Canvas layer
    pub const CANVAS: usize = 3;

    pub fn new(depth: usize, nodes: &'a mut [Node], l2_nodes: &'a mut [NodeL2]) -> Self {
        // Set leaf node
        nodes[1].flag = -2;
        nodes[1].children = [1; 8];
        // first free node head
        nodes[0].flag = -1;
        nodes[0].children[0] = 3;
        // Chain holders
        // First 2 nodes are not holders, the rest is holders
        // TODO: Decouple chaining holders and validating/creating layer
        // TODO: clear every node up
        for (i, free) in nodes.iter_mut().enumerate().skip(3) {
            // Mark as holder
            free.flag = -1;
            // Set link to next holder
            free.children[0] = i as u32 + 1;
        }

        nodes.last_mut().unwrap()[0] = 0;

        (nodes, l2_nodes, depth).into()
    }

    pub fn free(&self) -> usize {
        let mut idx = 0;
        let mut free = 0;
        loop {
            free += 1;
            idx = self[idx][0] as usize;
            if idx == 0 {
                break;
            }
            assert!(free < self.nodes.len())
        }

        free
    }

    /// Deallocate node with holder-pool
    pub fn deallocate_node(&mut self, node_idx: usize) {
        // Append empty linked list
        self[node_idx].flag = -1;
        self[node_idx].children[0] = self.nodes[0].children[0];

        // Change the actual head to new one
        self[0].children[0] = node_idx as u32;
    }

    /// Allocate node from holder-pool
    pub fn allocate_node(&mut self) -> usize {
        self.allocate_node_from(Node::default())
    }
    /// Allocate node from pool from given node
    pub fn allocate_node_from(&mut self, node: Node) -> usize {
        if self.nodes[0].children[0] != 0 {
            // Taking node linked by head
            let allocated_idx = self[0][0] as usize;
            let allocated_node = &mut self[allocated_idx];
            // Safety check
            assert_eq!(allocated_node.flag, -1);
            // Setting link from allocated_idx to head
            self[0][0] = allocated_node[0];
            // Set
            self[allocated_idx] = node;
            allocated_idx
        } else {
            panic!("You are out of holder-nodes");
        }
    }

    pub fn iter_fork<C: FnMut(&mut ForkIterProps)>(&self, mut fork_idx: usize, callback: &mut C) {
        if !self[fork_idx as usize].is_fork() {
            panic!()
        }

        loop {
            let fork = &self[fork_idx as usize];
            for voxel_id_idx in 0..4 {
                let (found_voxel_id, found_node_idx) =
                    (fork[voxel_id_idx * 2], fork[(voxel_id_idx * 2) + 1]);
                if found_voxel_id == 0 {
                    return;
                }
                let mut props = ForkIterProps {
                    drop: false,
                    voxel_id: found_voxel_id as usize,
                    node_idx: found_node_idx as usize,
                };
                callback(&mut props);

                if props.drop {
                    return;
                }
            }
            let next_opt = fork.flag;
            if next_opt > 0 {
                // Switch context until found or run out of links to branches
                fork_idx = next_opt as usize;
            } else if next_opt == -3 {
                return;
            } else {
                panic!();
            }
        }
    }

    /// Allows to work without even thinking about forks
    /// Usefull if node points to fork, but this abstracts it, and just returns idx to node below this all
    pub fn set_child(
        &mut self,
        from_idx: usize,
        voxel_id: u32,
        child_idx: usize,
        level: usize,
        fork_level: usize,
    ) -> usize {
        if voxel_id == 0 {
            panic!();
        }

        let node = &mut self[from_idx];

        // If children of node on from_idx should point to fork
        if level == fork_level + 1 {
            let node_below_idx = node.children[child_idx];

            if node_below_idx == 0 {
                // Fork does not exist, lets change that
                let new_branch_idx = self.allocate_node();

                let fork = Node {
                    flag: -3,
                    children: [voxel_id, new_branch_idx as u32, 0, 0, 0, 0, 0, 0],
                };

                let fork_idx = self.allocate_node_from(fork);

                self[from_idx].children[child_idx] = fork_idx as u32;

                return new_branch_idx;
            } else {
                let mut fork_idx = node_below_idx;

                loop {
                    let fork = &mut self[fork_idx as usize];
                    for voxel_id_idx in 0..4 {
                        let found_voxel_id = fork[voxel_id_idx * 2];
                        if found_voxel_id == 0 {
                            // Nothing here, lets fill it
                            // So we can allocate our new node to voxel_id
                            let new_branch_idx = self.allocate_node();

                            self[fork_idx as usize][voxel_id_idx * 2] = voxel_id;
                            self[fork_idx as usize][(voxel_id_idx * 2) + 1] = new_branch_idx as u32;

                            return new_branch_idx;
                        }
                        if found_voxel_id == voxel_id {
                            // Bravo, we found idx to forward!
                            return fork[(voxel_id_idx * 2) + 1] as usize;
                        }
                    }
                    let next_opt = fork.flag;
                    if next_opt > 0 {
                        // Switch context until found or run out of links to branches
                        fork_idx = next_opt as u32;
                    } else {
                        // If you are at this point, its only possible if there is not place left
                        // To fix that, lets extend it with new fork
                        // Fork does not exist, lets change that
                        let new_branch_idx = self.allocate_node();

                        let fork = Node {
                            flag: -3,
                            children: [voxel_id, new_branch_idx as u32, 0, 0, 0, 0, 0, 0],
                        };

                        let fork_id = self.allocate_node_from(fork);

                        self[fork_idx as usize].flag = fork_id as i32;

                        return new_branch_idx;
                    }
                }
            }
        } else {
            let node_below_idx = node.children[child_idx];
            if node_below_idx == 0 {
                let new_child_idx = self.allocate_node();

                self[from_idx].children[child_idx] = new_child_idx as u32;
                return new_child_idx;
            } else {
                return node.children[child_idx] as usize;
            }
        }
    }
}
/// A little helper
/// let layer: Layer = (nodes, l2_nodes, depth).into();
impl<'a> From<(&'a mut [Node], &'a mut [NodeL2], usize)> for Layer<'a> {
    fn from(value: (&'a mut [Node], &'a mut [NodeL2], usize)) -> Self {
        Layer {
            depth: value.2,
            level_2: value.1,
            nodes: value.0,
        }
    }
}

impl<'a> Index<usize> for Layer<'a> {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'a> IndexMut<usize> for Layer<'a> {
    fn index_mut(&mut self, index_mut: usize) -> &mut Self::Output {
        &mut self.nodes[index_mut]
    }
}
