use std::collections::HashMap;

use super::{
    graph::{Branch, Graph, Idx},
    level::GLevel,
    lookup_level::LookupLevel,
    shared::Shared,
};

impl Graph {
    /// Creates new graph with capacity on each level
    /// Its empty graph without any entries.
    pub fn new(depth: u8) -> Self {
        let mut levels = vec![
            GLevel {
                nodes: vec![
                    // Reserve with 999 nodes
                    Branch {
                        ident: 9,
                        children: [999; 8]
                    }
                ],
                empty_head: 0,
            };
            depth as usize + 1
        ];

        // levels[depth as usize].nodes.push(Branch::default());
        // Push leaf branch
        levels[0].nodes.push(Branch {
            ident: 3,
            children: [1; 8],
        });

        Graph {
            depth: depth as u32,
            levels,
            lookup_levels: vec![LookupLevel::default(); depth as usize + 1],
        }
    }

    /// Creates new entry and returns its handle id
    pub fn new_entry(&mut self) -> Idx {
        // Cache nodes
        let nodes = &mut self.levels[self.depth as usize].nodes;

        let idx = nodes.len();

        // Insert new node on
        nodes.push(Branch::default());
        idx
    }
    /// Create or return existing entry
    pub fn entry(&mut self, id: Idx) -> Idx {
        while self.levels[self.depth as usize].nodes.len() <= id {
            self.new_entry();
        }

        id
    }

    pub fn entries(&self) -> usize {
        self.levels[self.depth as usize].nodes.len()
    }
    /// Returns index to created branch, or to empty node
    pub fn add_branch(&mut self, level: u8) -> usize {
        // Cache
        let level = &mut self.levels[level as usize];

        if level.empty_head != 0 {
            // Taking the head of the chain to use
            let return_idx = level.empty_head;
            // Changing head to next node in empty chain
            level.empty_head = level[return_idx].children[0] as usize;
            // Clear branch
            level[return_idx] = Branch::default();
            return_idx
        } else {
            // Just create a branch
            level.nodes.push(Branch::default());
            level.nodes.len() - 1
        }
    }
    pub fn depth(&self) -> u8 {
        self.depth as u8
    }

    pub fn size(&self) -> u32 {
        1 << (self.depth())
    }
}

#[test]
fn new_graph() {
    let graph = Graph::new(2);
    dbg!(graph);
}
