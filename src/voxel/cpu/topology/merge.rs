use std::{collections::HashMap, ops::Range};

use glam::UVec3;

use crate::voxel::cpu::{traverse::TrProps, utils::lvl_to_size::lvl_to_size};

use super::{
    graph::{Branch, Graph},
    shared::Shared,
};

impl Graph {
    pub fn merge_segment(&mut self, position: UVec3, segment_lvl: u8) {
        for lvl in 1..(segment_lvl) {
            let entries = self.entries();
            let mut to_change = vec![];
            for entry in 1..entries {
                if let Some(idx) =
                    self.get_node(segment_lvl, position * lvl_to_size(segment_lvl), entry)
                {
                    Graph::traverse_from_unpositioned(&self.levels, idx, segment_lvl, |p| {
                        if let TrProps::Branch {
                            parent,
                            node,
                            level,
                            children,
                            ..
                        } = p
                        {
                            if node == 0 {
                                panic!("Node can not be 0");
                            }

                            if level == lvl as u8 {
                                if let Some(sh_idx) =
                                    self.lookup_levels[lvl as usize].links.get(children)
                                {
                                    to_change.push((node, parent, *sh_idx));
                                } else {
                                    self.lookup_levels[lvl as usize]
                                        .links
                                        .insert(children.clone(), node);
                                }

                                return false;
                            } else {
                                return true;
                            }
                        }
                        true
                    });
                }
            }
            for (current_idx, parent_idx, new_idx) in to_change {
                let parent_nodes = &mut self.levels[lvl as usize + 1].nodes;
                for child in &mut parent_nodes[parent_idx].children {
                    if *child == current_idx as u32 {
                        *child = new_idx as u32;
                    }
                }
                let level = &mut self.levels[lvl as usize];

                // Append empty linked list
                level.nodes[current_idx].ident = -1;

                // Back up currently head of empty chain
                let empty_head_idx = level.empty_head;

                // Change the actual head to new one
                level.empty_head = current_idx;

                if empty_head_idx != 0 {
                    // Link to prev empty head / to the rest of the chains
                    level.nodes[current_idx].children[0] = empty_head_idx as u32;
                } else {
                    // Clean up from last use
                    level.nodes[current_idx].children[0] = 0;
                }
            }
        }
    }

    /// Performing full merge. Takes more time, better result
    pub fn merge(&mut self) {
        // for lvl in 1..(self.depth) {
        //     // (node idx, node idx in shared)
        //     // let mut to_link = Vec::with_capacity(self.levels[lvl as usize].nodes.len());

        //     for (idx, node) in self.levels[lvl as usize].nodes.iter_mut().enumerate() {
        //         let lookup = &mut self.lookup_levels[lvl as usize].links;

        //         // // There is already node in shared store.
        //         // if let Some(link) = shared.level_caches[lvl as usize].map.get(node) {
        //         //     // Marking it as a link to shared storage
        //         //     node.ident = 1;
        //         //     // Setting links id
        //         //     node.children[0] = *link;
        //         // }
        //         // if there is no same node in shared, but there is same node on level
        //         if let Some(sh_idx) = lookup.get(node) {
        //             // // Create new entry in shared storage
        //             // let shared_level = &mut shared.levels[lvl as usize];
        //             // shared_level.nodes.push(node.clone());
        //             // // The len - 1 is its idx
        //             // let sh_idx = shared_level.nodes.len() - 1;

        //             // Marking this as a link
        //             node.ident = 2;
        //             // If its link, first child is link address.
        //             node.children[0] = *sh_idx as u32;

        //             // // We cant set another node to link from here.
        //             // // Cuz we alreade borrowed mut ref to node.
        //             // // So we will store it for later and perform "linking" after.
        //             // to_link.push((*local_link_idx, sh_idx));
        //         }
        //         // Its unique node for now
        //         else {
        //             // Add this node to lookup table
        //             // If there will be the same node on this level
        //             // We could go back to it and link to shared storage
        //             lookup.insert(node.clone(), idx);
        //         }
        //     }

        //     // // Create links for nodes in lookup table
        //     // for (idx, sh_idx) in to_link {
        //     //     // Caching nodes
        //     //     let nodes = &mut self.levels[lvl as usize].nodes;
        //     //     let node = &mut nodes[idx];

        //     //     node.ident = 1;
        //     //     node.children[0] = sh_idx as u32;
        //     // }

        //     // Apply links that were created during this stage.
        //     // Otherwise we would have no result while merging level above
        //     self.levels[lvl as usize].collapse_links();
        // }
    }

    /// Fast merge, e.g. Partial merge. Merging only ranged voxels into shared merge nodes.
    /// For better result use `merge`
    pub fn fast_merge(&mut self, shared: &mut Shared, merge_range: Range<usize>) {}

    pub fn get_level() {}
}
