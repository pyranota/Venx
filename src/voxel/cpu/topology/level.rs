use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::graph::{Branch, Idx};

#[derive(Default, Clone, Debug)]
pub struct GLevel {
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
        &self.nodes[index - 1] // -1
    }
}

impl IndexMut<usize> for GLevel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index - 1] // -1
    }
}
