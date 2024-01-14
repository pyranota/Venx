use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::graph::{Branch, Idx};

#[derive(Default, Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct GLevel {
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
    pub nodes: Vec<Branch>,
    /// Link to first node which is empty (flag == -1)
    /// If there is no empty nodes its 0
    pub empty_head: Idx,
}

impl GLevel {
    /// Index the node by its IDX. If you are indexing at link node, it will return referenced node.
    pub fn index_node_mut(&mut self, mut idx: Idx) -> &mut Branch {
        while self.nodes[idx].ident == 2 {
            idx = self.nodes[idx].children[0] as usize
        }
        &mut self.nodes[idx]
    }
    /// Index the node by its IDX. If you are indexing at link node, it will return referenced node.
    pub fn index_node(&self, mut idx: Idx) -> &Branch {
        while self.nodes[idx].ident == 2 {
            idx = self.nodes[idx].children[0] as usize
        }
        &self.nodes[idx]
    }
    /// Returns a table of idx of node to new idx
    pub fn new_redir_table(&mut self) -> HashMap<Idx, Idx> {
        let mut table: HashMap<Idx, Idx> = HashMap::new();

        // Iterating over all nodes on level to find out link nodes
        // Might be removed if graph branches are double linked
        // It looks slow, but it can be actually not. TODO! find it out
        for (i, node) in self.nodes.iter().enumerate().skip(1) {
            if node.ident == 2 {
                table.insert(i, node.children[0] as usize);
            }
        }

        table
    }
    /// Merging all links
    /// If there is any branch with flag == 2 it will make it blank/empty node (with flag == -1).
    pub fn collapse_links(&mut self) {
        let table = self.new_redir_table();

        // Looping over all nodes in level and looking for every child of every node
        for node in self.nodes.iter_mut() {
            if node.ident == 0 {
                for child in &mut node.children {
                    if child != &0 {
                        if let Some(new) = table.get(&(*child as usize)) {
                            *child = *new as u32;
                        }
                    }
                }
            }
        }
    }
}

impl Index<usize> for GLevel {
    type Output = Branch;

    fn index(&self, index: usize) -> &Self::Output {
        // Cant index 0
        if index == 0 {
            panic!("You cant index 0!");
        }
        &self.nodes[index] // -1
    }
}

impl IndexMut<usize> for GLevel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Cant index 0
        if index == 0 {
            panic!("You cant index 0!");
        }
        &mut self.nodes[index] // -1
    }
}
