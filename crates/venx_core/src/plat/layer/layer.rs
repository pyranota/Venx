use core::ops::{Index, IndexMut};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::UVec3;

use crate::{
    plat::{node::Node, op::traverse::Props},
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

impl<'a> Layer<'a> {
    pub fn new(depth: usize, nodes: &'a mut [Node], entries: &'a mut [usize]) -> Self {
        // Set leaf node
        nodes[1].flag = 3;
        nodes[1].children = [1; 8];
        // Set reserved node
        nodes[0].flag = 9;
        // Chain holders
        // First 2 nodes are not holders, the rest is holders
        // TODO: Make last holder reference 0 instaed of non-existing node
        // TODO: Decouple chaining holders and validating/creating layer
        // TODO: clear every node up
        for (i, holder) in nodes.iter_mut().enumerate().skip(2) {
            // Mark as holder
            holder.flag = -1;
            // Set link to next holder
            holder.children[0] = i as u32 + 1;
        }

        // Set holder head
        entries[0] = 2;

        Layer {
            depth,
            entries,
            //       holder_head: 2, // 0 Reserved, 1 Leaf, 2 Holder, 3 Holder, ... 5_000 Holder ...
            nodes,
            freezed: false,
        }
    }

    /// Specify `entry` just to forward it to callback's props. It is not used elsewhere
    /// `from_node_position` also just to forward, you can ignore these arguments
    /// To speed up set position to None, but it wont display any position information
    pub fn traverse<F>(
        &self,
        entry: u32,
        from_node_idx: usize,
        from_node_position: Option<UVec3>,
        from_level: usize,
        mut callback: &mut F,
    ) where
        F: FnMut(Props) -> bool,
    {
        // assert_ne!(from_node_idx, 0);

        visit_node(
            self,
            from_node_idx,
            0,
            from_node_position,
            from_level,
            entry,
            &mut callback,
        );
        fn visit_node<F>(
            layer: &Layer,
            idx: usize,
            parent_idx: usize,
            node_position_opt: Option<UVec3>,
            level: usize,
            entry: u32,
            callback: &mut F,
        ) where
            F: FnMut(Props) -> bool,
        {
            let node = layer[idx];

            if !callback(Props {
                position: &node_position_opt,
                parent_idx: &parent_idx,
                node: &node,
                entry,
                level,
            }) || level == 0
            {
                return;
            }
            // WATCH: Careful, might be a bug here. Prev: let size = node.size() / 2;
            let size = l2s(level) / 2;

            // Iterate over all children. Order cannot be changed.
            for (i, child_id) in node.children.iter().enumerate() {
                if *child_id != 0 {
                    // TODO: Profile, it might be slow to handle position this way
                    let mut local_node_position_opt: Option<UVec3> = node_position_opt;
                    if let Some(node_position) = &mut local_node_position_opt {
                        *node_position += Node::get_child_position(i as u32) * size;
                    }

                    visit_node(
                        layer,
                        *child_id as usize,
                        idx,
                        local_node_position_opt,
                        level - 1,
                        entry,
                        callback,
                    );
                }
            }
        }
    }

    pub(crate) fn get_node(
        &self,
        mut position: UVec3,
        level: usize,
        entry: usize,
    ) -> Option<usize> {
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = None;

        let mut idx = self.entries[entry];

        while current_level > level {
            let child_index = Node::get_child_index(position, current_level - 1);

            let child_id = self[idx].children[child_index];

            if child_id != 0 {
                idx = child_id as usize;
                found_idx = Some(child_id as usize);
            } else {
                return None;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }
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

    pub fn test_entry_wrapper(&mut self, idx: usize) -> usize {
        self.entry(idx)
    }

    pub fn just_method(&mut self) -> usize {
        self.depth
    }
    /// Allocate node from holder-pool
    pub fn allocate_node(&mut self) -> usize {
        if self.entries[0] != 0 {
            // Taking the head of the chain to use
            let return_idx = self.entries[0];
            // Changing head to next node in empty chain
            self.entries[0] = self[return_idx].children[0] as usize;
            // Clear branch
            self[return_idx] = Node::default();
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
