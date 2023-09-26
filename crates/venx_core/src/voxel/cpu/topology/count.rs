use std::ops::Range;

use crate::voxel::cpu::facade::Idx;

use super::graph::Graph;

impl Graph {
    /// Counts attribute index until given level
    /// Uses for block-typed oprations, untyped will not use that
    pub fn count_children(&self, idx: Idx, upto: usize) -> u32 {
        let node = &self.nodes[idx];
        let mut counter = 0;

        let children = &node.get_branch().unwrap().children;

        for i in 0..upto {
            if children[i] != 0 {
                counter += &self.nodes[children[i] as usize]
                    .get_branch()
                    .unwrap()
                    .attr_count;
            }
        }

        counter
    }
}
