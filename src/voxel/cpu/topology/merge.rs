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
            // Lookup table. Used for finding same nodes on level.
            let mut lookup: HashMap<Branch, usize> = HashMap::new();

            // (node idx, node idx in shared)
            let mut to_link = Vec::with_capacity(self.levels[lvl as usize].nodes.len());

            for (idx, node) in self.levels[lvl as usize].nodes.iter_mut().enumerate() {
                // There is already node in shared store.
                if let Some(link) = shared.level_caches[lvl as usize].map.get(node) {
                    // Marking it as a link to shared storage
                    node.ident = 1;
                    // Setting links id
                    node.children[0] = *link;
                }
                // if there is no same node in shared, but there is same node on level
                else if let Some(local_link_idx) = lookup.get(node) {
                    // Create new entry in shared storage
                    let shared_level = &mut shared.levels[lvl as usize];
                    shared_level.nodes.push(node.clone());
                    // The len - 1 is its idx
                    let sh_idx = shared_level.nodes.len() - 1;

                    // Marking this as a link
                    node.ident = 2;
                    // If its link, first child is link address.
                    node.children[0] = sh_idx as u32;

                    // We cant set another node to link from here.
                    // Cuz we alreade borrowed mut ref to node.
                    // So we will store it for later and perform "linking" after.
                    to_link.push((*local_link_idx, sh_idx));
                }
                // Its unique node for now
                else {
                    // Add this node to lookup table
                    // If there will be the same node on this level
                    // We could go back to it and link to shared storage
                    lookup.insert(node.clone(), idx);
                }
            }

            // Create links for nodes in lookup table
            for (idx, sh_idx) in to_link {
                // Caching nodes
                let nodes = &mut self.levels[lvl as usize].nodes;
                let node = &mut nodes[idx];

                node.ident = 1;
                node.children[0] = sh_idx as u32;
            }

            // Apply links that were created during this stage.
            // Otherwise we would have no result while merging level above
            self.levels[lvl as usize].collapse_links();
        }
    }

    /// Fast merge, e.g. Partial merge. Merging only ranged voxels into shared merge nodes.
    /// For better result use `merge`
    pub fn fast_merge(&mut self, shared: &mut Shared, merge_range: Range<usize>) {}

    pub fn get_level() {}
}
