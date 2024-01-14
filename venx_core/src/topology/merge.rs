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
}
