use std::ops::Range;

use super::graph::{Graph, Idx};

impl Graph {
    /// Counts attribute index until given level
    /// Uses for block-typed oprations, untyped will not use that
    pub fn count_children(&self, idx: Idx, upto: usize) -> u32 {
        todo!()
        // let node = &self.nodes[idx];
        // let mut counter = 0;

        // let children = &node.children;

        // // for i in 0..upto {
        // //     if children[i] != 0 {
        // //         counter += &self.nodes[children[i] as usize].attr_count;
        // //     }
        // // }

        // counter
    }
}
