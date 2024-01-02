use std::{collections::HashMap, ops::Range};

use super::{
    graph::{Branch, Graph},
    shared::Shared,
};

impl Graph {
    /// Performing full merge. Takes more time, better result
    pub fn merge(&mut self, shared: &mut Shared) {
        for lvl in 1..(self.depth) {
            // Merge level
            let mut lookup: HashMap<Branch, u32> = HashMap::new();
            for (i, node) in self.levels[lvl as usize].nodes.iter().enumerate() {
                if let Some(link) = shared.level_caches[lvl as usize].map.get(node) {
                    todo!()
                } else if let Some(local_link) = lookup.get(node) {
                    todo!()
                } else {
                    todo!()
                }
            }
            //
        }
    }

    /// Fast merge, e.g. Partial merge. Merging only ranged voxels into shared merge nodes.
    /// For better result use `merge`
    pub fn fast_merge(&mut self, shared: &mut Shared, merge_range: Range<usize>) {}

    pub fn get_level() {}
}
