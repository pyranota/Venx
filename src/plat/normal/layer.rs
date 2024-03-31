use std::collections::HashMap;

use async_trait::async_trait;
use log::{info, warn};

use venx_core::plat::{node::Node, node_l2::NodeL2, op::get::GetNodeResult};

use crate::plat::interfaces::layer::LayerInterface;

use super::cpu_plat::CpuPlat;
#[async_trait]
impl LayerInterface for CpuPlat {
    async fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        self.with_raw_plat_mut(|plat| plat[layer].set(position.to_array().into(), ty as u32));
    }

    fn compress(
        &mut self,
        layer: usize,
        position: glam::UVec3,
        level: u32,
        lookup_tables: &mut Vec<HashMap<Node, usize>>,
        lookup_table_l2: &mut HashMap<NodeL2, usize>,
    ) {
        for lvl in 2..=3 {
            let mut to_change = vec![];

            self.with_raw_plat_mut(|plat| {
                let layer = &mut plat[layer];

                if lvl == 2 {
                    layer.traverse(position.to_array().into(), 0..=(level as usize), |p| {
                        if p.level == lvl {
                            let node = &layer.level_2[p.node_idx];
                            if let Some(shared_idx) = lookup_table_l2.get(node) {
                                if *shared_idx != p.node_idx {
                                    to_change.push((p.node_idx, *p.parent_idx, *shared_idx));
                                }
                            } else {
                                lookup_table_l2.insert(*node, p.node_idx);
                            }
                        }
                    });
                } else {
                    layer.traverse(position.to_array().into(), 0..=(level as usize), |p| {
                        if p.level == lvl {
                            let mut node = layer[p.node_idx];
                            node.flag = 0;
                            if let Some(shared_idx) = lookup_tables[lvl].get(&node) {
                                if *shared_idx != p.node_idx {
                                    to_change.push((p.node_idx, *p.parent_idx, *shared_idx));
                                }
                            } else {
                                lookup_tables[lvl].insert(node, p.node_idx);
                            }
                        }
                    });
                }

                let mut deallocated = 0;

                'changing: for (current_idx, parent_idx, new_idx) in to_change {
                    // let parent_children = &mut self.levels[lvl as usize + 1].nodes;
                    if lvl == 4 {
                        for child in &mut layer[parent_idx].children.iter_mut().skip(1).step_by(2) {
                            if *child == current_idx as u32 {
                                *child = new_idx as u32;
                                deallocated += 1;
                                if lvl == 2 {
                                    layer.deallocate_node::<NodeL2>(current_idx);
                                } else {
                                    layer.deallocate_node::<Node>(current_idx);
                                }

                                continue 'changing;
                            }
                        }
                    } else {
                        for child in &mut layer[parent_idx].children {
                            if *child == current_idx as u32 {
                                *child = new_idx as u32;
                                deallocated += 1;
                                if lvl == 2 {
                                    layer.deallocate_node::<NodeL2>(current_idx);
                                } else {
                                    layer.deallocate_node::<Node>(current_idx);
                                }

                                continue 'changing;
                            }
                        }
                    }
                }
                //dbg!(layer.free(), layer.free_l2());
                if deallocated == 0 {
                    warn!("No deallocations");
                }
            });
        }

        //if position.x > 5 && position.z > 5 {
        info!(
            "Free space on layer: \n l2:   {} \n rest: {}",
            self.borrow_raw_plat()[layer].free_l2(),
            self.borrow_raw_plat()[layer].free()
        );
        //}
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<GetNodeResult> {
        let res = self.borrow_raw_plat().get_voxel(position.to_array().into());

        if res.is_some() {
            Some(res)
        } else {
            None
        }
    }
}
