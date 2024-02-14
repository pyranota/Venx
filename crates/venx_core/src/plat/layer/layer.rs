use core::ops::{Index, IndexMut};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{
        node::{Node, NodeAddr},
        op::{get::GetNodeResult, traverse::Props},
        stack::EStack,
    },
    utils::l2s,
};

#[derive(PartialEq)]
pub struct Layer<'a> {
    // TODO: move to RawPlat metadata
    /// Can be edited or not
    pub freezed: bool,
    /// Synced depth with RawPlat
    pub depth: usize,
    // Maybe use custom struct Entry instead of usize?
    pub entries: &'a mut [usize],
    // TODO: move to entries[0]
    // pub meta: LayerMeta,
    /// Link to first node which is empty (flag == -1)
    /// If there is no empty nodes its 0
    // pub holder_head: usize,
    /// Every node on level(depth) is entry node
    /// Each entry represents root of graph
    /// That means, that in single `Graph` struc, you can have multiple graphs
    /// That is used to have voxel types in graph
    /// All graphs are merged
    /// By creating new entry you create new graph

    /// Keep in mind that anything on 0 is reserved and not usable
    /// You can identify this types of nodes with 9 in every field of it
    /// This is in that way because if there would be node at 0 index,
    /// that would conflict with 0 as "no child" interpretation
    pub nodes: &'a mut [Node],
}

pub struct ForkIterProps {
    drop: bool,
    voxel_id: usize,
    node_idx: usize,
}

impl<'a> Layer<'a> {
    pub fn new(depth: usize, nodes: &'a mut [Node], entries: &'a mut [usize]) -> Self {
        // Set leaf node
        nodes[1].flag = -2;
        nodes[1].children = [1; 8];
        // first free node head
        nodes[0].flag = -1;
        nodes[0].children[0] = 3;
        // Chain holders
        // First 2 nodes are not holders, the rest is holders
        // TODO: Make last holder reference 0 instaed of non-existing node
        // TODO: Decouple chaining holders and validating/creating layer
        // TODO: clear every node up
        for (i, free) in nodes.iter_mut().enumerate().skip(3) {
            // Mark as holder
            free.flag = -1;
            // Set link to next holder
            free.children[0] = i as u32 + 1;
        }

        Layer {
            depth,
            entries,
            //       // 0 Free node head, 1 Leaf, 2 Root, 3 Holder, ... 5_000 Holder ...
            nodes,
            freezed: false,
        }
    }

    pub(crate) fn get_node(
        &self,
        mut position: UVec3,
        level: usize,
        voxel_id_opt: Option<usize>,
    ) -> GetNodeResult {
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            let child_index = Node::get_child_index(position, current_level - 1);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    return res;
                }
            } else {
                return GetNodeResult::None();
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            // if let Some(needed_voxel_id) = voxel_id_opt {
            //     if voxel_id != needed_voxel_id {

            //     }
            // }

            let mut size = size;
            let mut position = position.clone();
            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = Node::get_child_index(position, current_level - 1);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    size /= 2;
                    position %= size;
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx
    }

    pub(crate) fn get_node_with_addr(
        &self,
        addr: &NodeAddr,
        level: usize,
        voxel_id_opt: Option<usize>,
    ) -> GetNodeResult {
        let mut current_level = self.depth as usize;

        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            let child_index = addr.get_idx(current_level);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    return res;
                }
            } else {
                return GetNodeResult::None();
            }
            {
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            // if let Some(needed_voxel_id) = voxel_id_opt {
            //     if voxel_id != needed_voxel_id {

            //     }
            // }

            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = addr.get_idx(current_level);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx
    }

    /// Get ref to root of existing subtree, or create new
    pub fn entry(&mut self, idx: usize) -> usize {
        if self.entries[idx] != 0 {
            return self.entries[idx];
        } else {
            let new = self.allocate_node();
            self.entries[idx] = new;
            return new;
        }
    }

    /// Allocate node from holder-pool
    pub fn allocate_node(&mut self) -> usize {
        self.allocate_node_from(Node::default())
    }
    /// Allocate node from pool from given node
    pub fn allocate_node_from(&mut self, node: Node) -> usize {
        if self.nodes[0].children[0] != 0 {
            // Taking the head of the chain to use
            let return_idx = self.nodes[0].children[0] as usize;
            // Changing head to next node in empty chain
            self.nodes[0].children[0] = self[return_idx].children[0];
            // Clear branch
            self[return_idx] = node;
            return_idx
        } else {
            panic!("You are out of holder-nodes");
        }
    }
    /// Returns slice of sorted by priority voxel types existing in specified region
    /// If position is None, than it returns all entries in layer
    pub fn get_entries_in_region(&'a self, position: Option<UVec3>) -> &'a [usize] {
        // TODO do actual algorithm
        // For now just return all voxel types in layer

        self.entries
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
