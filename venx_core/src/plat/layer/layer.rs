use core::ops::{Index, IndexMut};

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::UVec3;

use crate::plat::node::Node;

#[derive(Debug)]
pub struct Layer<'a> {
    /// Can be edited or not
    pub freezed: bool,
    pub depth: u8,
    // Maybe use custom struct Entry instead of usize?
    pub entries: &'a mut [usize],
    // pub meta: LayerMeta,
    /// Link to first node which is empty (flag == -1)
    /// If there is no empty nodes its 0
    pub holder_head: usize,
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
    pub fn new(depth: u8, nodes: &'a mut [Node], entries: &'a mut [usize]) -> Self {
        // Set leaf node
        nodes[1].flag = 3;
        nodes[1].children = [1; 8];
        // Set reserved node
        nodes[0].flag = 9;
        // Chain holders
        // First 2 nodes are not holders, the rest is holders
        for (i, holder) in nodes.iter_mut().enumerate().skip(2) {
            // Mark as holder
            holder.flag = -1;
            // Set link to next holder
            holder.children[0] = i as u32 + 1;
        }

        Layer {
            depth,
            entries,
            holder_head: 2, // 0 Reserved, 1 Leaf, 2 Holder, 3 Holder, ... 5_000 Holder ...
            nodes,
            freezed: false,
        }
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
        if self.holder_head != 0 {
            // Taking the head of the chain to use
            let return_idx = self.holder_head;
            // Changing head to next node in empty chain
            self.holder_head = self[return_idx].children[0] as usize;
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
