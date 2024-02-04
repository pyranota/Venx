use core::ops::{Index, IndexMut};

use crate::plat::node::Node;

#[derive(Debug)]
pub struct Layer {
    /// Can be edited or not
    pub freezed: bool,
    pub entries: *mut [usize],
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
    nodes: *mut [Node],
}

impl Layer {
    pub fn new<const LEN: usize>(name: &str, depth: u8) -> Self {
        let mut nodes = [Node::default(); LEN];
        // Set leaf node
        nodes[1].flag = 3;
        nodes[1].children = [1; 8];
        // Set reserved node
        nodes[0].flag = 9;
        Layer {
            entries: &mut [],
            holder_head: 0,
            nodes: &mut nodes,
            freezed: false,
        }
    }
    /// Get ref to root of existing subtree, or create new
    pub fn entry(&mut self, idx: usize) -> usize {
        todo!()
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
}

impl Index<usize> for Layer {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for Layer {
    fn index_mut(&mut self, index_mut: usize) -> &mut Self::Output {
        todo!()
    }
}
