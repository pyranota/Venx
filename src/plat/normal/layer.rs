use std::{borrow::BorrowMut, collections::HashMap};

use log::info;
use venx_core::{
    glam::UVec3,
    l2s,
    plat::{node::Node, op::get::GetNodeResult},
    utils::Grid,
};

use crate::plat::interfaces::layer::LayerInterface;

use super::cpu_plat::CpuPlat;

impl LayerInterface for CpuPlat {
    fn set_segment<const SIZE: usize>(
        &mut self,
        layer: usize,
        segment: Grid<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        self.with_raw_plat_mut(|plat| plat[layer].set(position.to_array().into(), ty as u32));
    }

    fn compress(
        &mut self,
        layer: usize,
        position: glam::UVec3,
        level: u32,
        lookup_tables: &mut Vec<HashMap<Node, usize>>,
    ) {
        for lvl in 1..(4) {
            let mut to_change = vec![];

            // TODO: Cache it

            self.with_raw_plat_mut(|plat| {
                let layer = &mut plat[layer];

                let node_idx = layer.get_node_idx_gpu(
                    venx_core::glam::UVec3::from_array((position * l2s(level as usize)).to_array()),
                    level as usize,
                    None,
                ); // position * l2s(region_level)

                if node_idx == 0 {
                    info!("Empty, not merging");
                    return;
                }

                layer.traverse(0, node_idx, UVec3::ZERO, false, level as usize, &mut |p| {
                    if p.level == lvl {
                        if let Some(shared_idx) = lookup_tables[lvl].get(p.node) {
                            if *shared_idx != p.node_idx {
                                to_change.push((p.node_idx, *p.parent_idx, *shared_idx));
                            }
                        } else {
                            lookup_tables[lvl].insert(*p.node, p.node_idx);
                        }

                        p.drop_tree = true;
                    }
                });

                'changing: for (current_idx, parent_idx, new_idx) in to_change {
                    // let parent_children = &mut self.levels[lvl as usize + 1].nodes;
                    for child in &mut layer[parent_idx].children {
                        if *child == current_idx as u32 {
                            *child = new_idx as u32;
                            layer.deallocate_node(current_idx);
                            continue 'changing;
                        }
                    }
                }
            });
        }
        //
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<GetNodeResult> {
        let res = self.borrow_raw_plat().get_voxel(position.to_array().into());

        if res.is_some() {
            return Some(res);
        } else {
            return None;
        }
    }
}
