use std::collections::HashMap;

use super::{
    graph::{Branch, Graph, Idx},
    level::GLevel,
    shared::Shared,
};

impl Graph {
    // pub fn new(depth: u8) -> Self {
    //     let mut leaf = GBranch::new(0);
    //     leaf.attr_count = 1;
    //     Graph {
    //         head_holder_idx: 0,
    //         segment_level: 5,
    //         compression_level: 11,
    //         depth: depth as u32,
    //         nodes: vec![
    //             GNode::new_branch_from(GBranch::new(depth)), // Root
    //             GNode::new_branch_from(leaf),                // Leaf
    //         ],
    //     }
    // }

    /// Creates new graph with capacity on each level
    /// Its empty graph without any entries.
    pub fn new(depth: u8) -> Self {
        let mut levels = vec![
            GLevel {
                nodes: vec![
                    // Reserve with 999 nodes
                    Branch {
                        ident: 9,
                        children: [9; 8]
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
            shared: Shared::default(),
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
    // pub async fn load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<(Chunk, MeshSize)> {
    //     todo!()
    // }

    // pub async fn turbo_load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<Chunk> {
    //     todo!()
    // }

    // pub async fn set_voxel_cpu() {}

    // pub async fn complete_segment_cpu(&mut self, segment: Vec<Vec<Vec<u32>>>, position: UVec3) {
    //     todo!()
    // }

    // In future should use unused nodes
    pub fn add_branch(&mut self, level: u8, branch: Branch) -> usize {
        self.levels[level as usize].nodes.push(branch);
        self.levels[level as usize].nodes.len() - 1
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
