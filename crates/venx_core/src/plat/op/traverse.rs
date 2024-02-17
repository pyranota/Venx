use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat, stack::EStack},
    utils::l2s,
};

use super::{EntryOpts, LayerOpts};

/// Data-type used by traversal callbacks
#[derive(Clone)]
pub struct Props<'a> {
    /// Position of node in global 3d coords
    /// If no initial position was specified in `traverse` method, it will be local
    pub position: &'a UVec3,
    /// If false, than position is always UVec3::ZERO (Makes algorithm a bit faster)
    pub positioned: bool,
    /// Idx of parent node for given node. If 0, than there is no parents (works only for root node)
    pub parent_idx: &'a usize,
    /// Actual Node data
    pub node: &'a Node,
    /// Node idx
    pub node_idx: usize,
    /// Level of node
    pub level: usize,
    /// Forwarded entry index
    pub entry: u32,
    /// By default each callback prop has `drop_tree = false`.
    /// If you want to drop traversing of current node and all its children, set to `true`
    /// Be aware, it wont drop traversal of entire graph
    pub drop_tree: bool,
}

impl RawPlat<'_> {
    /// Traverse through all voxels in world specified in arguments
    /// Algorithm goes from bottom to up, meaning that some voxels can overlap, in that case works recent-right rule.
    /// Return false in callback to drop traversing of subtree
    pub fn traverse<F>(&self, layer_opts: LayerOpts, entry_opts: EntryOpts, callback: &mut F)
    where
        F: FnMut(&mut Props),
    {
        for layer_idx in 0..4 {
            self[layer_idx].traverse(
                0,
                // TODO: do something about this unsafe cringe
                2,
                UVec3::ZERO,
                true,
                self.depth,
                callback,
            );
        }
        // Iterate over all layers and nodes
        // self.opts(
        //     None,
        //     layer_opts,
        //     entry_opts,
        //     true,
        //     &mut |plat, (layer, layer_id), entry| {
        //         layer
        //         return None as Option<()>;
        //     },
        // );
    }

    pub fn traverse_unpositioned<F>(
        &self,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        callback: &mut F,
    ) where
        F: FnMut(&mut Props),
    {
        // Iterate over all layers and nodes
        self.opts(
            None,
            layer_opts,
            entry_opts,
            true,
            &mut |_plat, (layer, layer_id), entry| {
                layer.traverse(
                    entry,
                    entry as usize,
                    UVec3::ZERO,
                    false,
                    self.depth,
                    callback,
                );
                return None as Option<()>;
            },
        );
    }

    /// Traversing all nodes on all levels with voxel overlapping
    /// layers and voxels can overlap
    /// So if you specify a single layer, there are no overlaps
    /// Also region_position is just some value in global space within this region
    /// Dont traverse from level == depth, use normal `traverse`
    pub fn traverse_region<F>(
        &self,
        region_position: UVec3,
        region_level: usize,
        entry_opts: EntryOpts,
        layer_opts: LayerOpts,
        callback: &mut F,
    ) where
        F: FnMut(&mut Props),
    {
        let fork_level = 4;
        assert!(region_level > fork_level);

        for layer_idx in 0..4 {
            let layer = &self[layer_idx];

            let node_idx =
                layer.get_node_idx_gpu(region_position * l2s(region_level), region_level, None);

            if node_idx != 0 {
                layer.traverse(0, node_idx, UVec3::ZERO, true, region_level, callback)
            }
        }

        // self.opts(
        //     None,
        //     layer_opts,
        //     entry_opts,
        //     true,
        //     &mut |_plat, (layer, ..), entry| {
        //         // We need explicitly call it for all specified entries and layers. Otherwise it would find just one node with most priority.

        //         None as Option<()>
        //     },
        // );
    }
}

impl Layer<'_> {
    /// Depth-first traversal of layer.
    /// `entry: u32`, `from_node_position: UVec3` are used to adjust data in `Props`
    pub fn traverse<F>(
        &self,
        entry: u32,
        from_node_idx: usize,
        from_node_position: UVec3,
        positioned: bool,
        from_level: usize,
        callback: &mut F,
    ) where
        F: FnMut(&mut Props),
    {
        // TODO: use assert!
        if cfg!(feature = "bitcode_support") {
            assert_ne!(from_node_idx, 0);
        }

        // Emulate stack with max depth 21 (max graph depth)
        // Why? This code should compile to SpirV
        let mut stack: EStack<(
            /* 0 node_idx */
            usize,
            /* 1 parent_idx */
            usize,
            /* 2 node_position */
            UVec3,
            /* level */
            usize,
            /* voxel_id */
            usize,
            /* index (progress of iterator in specific node) */
            usize,
        )> = EStack::new((from_node_idx, 0, from_node_position, from_level, 0, 0));

        loop {
            // Read without pulling it
            let (node_idx, parent_idx, mut position, level, voxel_id, index) = stack.read();
            // Exit
            if *index > 7 && *level == from_level {
                break;
            }

            // Some cache going on here
            let node = &self[*node_idx];

            if node.is_fork() {
                if *index % 2 != 0 {
                    panic!()
                }

                // Out of bound
                if *index == 8 {
                    let flag = node.flag;
                    if flag > 0 {
                        // Switch to next fork in chain
                        *node_idx = flag as usize;
                        // Reset index
                        *index = 0;

                        continue;
                    } else if flag == -3 {
                        stack.pop();
                        continue;
                    } else {
                        panic!()
                    }
                }
                let voxel_id = &node.children[*index];
                let child_id = &node.children[*index + 1];

                *index += 2;

                if *child_id != 0 {
                    // if *index == 4 {
                    //     panic!();
                    // }
                    let (node_idx, level) = (node_idx.clone(), level.clone());

                    stack.push((
                        *child_id as usize,
                        node_idx,
                        position.clone(),
                        level,
                        *voxel_id as usize,
                        0,
                    ));

                    continue;
                } else {
                    // Exit fork chain
                    stack.pop();
                    continue;
                }
            }

            // Iterated over all children
            if *index > 7 {
                stack.pop();
                continue;
            }

            // Call for each enter once
            // If remove, it will call this callback 7 extra times
            if *index == 0 {
                let mut props = Props {
                    // TODO: Make use of positions
                    position: &position,
                    positioned,
                    parent_idx: &parent_idx,
                    node: &node,
                    node_idx: *node_idx,
                    entry: *voxel_id as u32,
                    level: *level,
                    drop_tree: false,
                };

                // let ret = callback(props);
                callback(&mut props);

                // Drop subtree traversal
                if props.drop_tree || *level == 0 {
                    stack.pop();
                    continue;
                }
            }

            let size = l2s(*level) / 2;

            // Actual node idx in layer.nodes
            let child_id = &node.children[*index];

            // Increment ahead, so if child_id == 0, it will still do some progress
            *index += 1;

            if *child_id != 0 && *level > 0 {
                // TODO: Profile, it might be slow to handle position this way
                if positioned {
                    position += Node::get_child_position(*index as u32 - 1) * size;
                }

                // TODO: Do we need this cache?
                let (node_idx, level, voxel_id) =
                    (node_idx.clone(), level.clone(), voxel_id.clone());

                stack.push((
                    *child_id as usize,
                    node_idx,
                    position.clone(),
                    level - 1,
                    voxel_id,
                    0,
                ));
            }
        }
    }
}
#[cfg(feature = "bitcode_support")]
#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;
    use crate::*;
    use core::borrow::{Borrow, BorrowMut};
    use std::{dbg, println};

    use alloc::{
        borrow::ToOwned,
        boxed::Box,
        vec::{self, Vec},
    };
    use rand::Rng;
    use spirv_std::glam::{uvec3, UVec3};

    use crate::{
        plat::{
            chunk::chunk::Chunk,
            node::{Node, NodeAddr},
            op::{EntryOpts, LayerOpts},
            raw_plat::{LayerIndex, RawPlat},
        },
        utils::l2s,
    };

    #[test]
    fn traverse_region() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            6,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );

        // Base
        plat[Base].set(uvec3(14, 14, 14), 1);
        plat[Base].set(uvec3(0, 0, 0), 2);
        plat[Base].set(uvec3(5, 15, 5), 3);
        plat[Base].set(uvec3(0, 10, 0), 1);

        // Canvas
        plat[Canvas].set(uvec3(15, 15, 15), 1);
        plat[Canvas].set(uvec3(0, 0, 0), 2);
        let mut seq = alloc::vec![];

        plat.traverse_region(
            UVec3::ZERO,
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |props| {
                if props.level == 0 {
                    seq.push(props.position.clone());
                }
            },
        );
        assert_eq!(
            seq,
            [
                uvec3(0, 10, 0),
                uvec3(14, 14, 14),
                uvec3(0, 0, 0),
                uvec3(5, 15, 5),
                uvec3(15, 15, 15),
                uvec3(0, 0, 0)
            ]
        );
    }

    #[test]
    fn traverse_check_props_node_idx() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        plat[0].set((0, 12, 8).into(), 22);
        plat[0].set((1, 12, 8).into(), 32);
        plat[0].set((2, 12, 8).into(), 52);
        plat[0].set((5, 12, 4).into(), 12);
        plat[0].set((1, 2, 11).into(), 2);

        // plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
        //     assert_eq!(
        //         plat[0].get_node_idx_gpu(*p.position, p.level, Some(p.entry as usize)),
        //         p.node_idx
        //     )
        // });
    }

    #[test]
    fn traverse_check_props_node() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        plat[0].set((0, 12, 8).into(), 22);
        plat[0].set((1, 12, 8).into(), 32);
        plat[0].set((2, 12, 8).into(), 52);
        plat[0].set((5, 12, 4).into(), 12);
        plat[0].set((1, 2, 11).into(), 2);

        // plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
        //     assert_eq!(
        //         &plat[0].nodes[plat[0].get_node_idx_gpu(*p.position, p.level, None)],
        //         p.node
        //     )
        // });
    }

    #[test]
    fn traverse_check_props_parent_idx() {
        quick_raw_plat!(plat, depth 6, len 100);

        plat[0].set((0, 12, 8).into(), 2);
        plat[0].set((1, 12, 8).into(), 2);
        plat[0].set((2, 12, 8).into(), 2);
        plat[0].set((5, 12, 4).into(), 2);
        plat[0].set((1, 2, 11).into(), 2);

        let mut seq = alloc::vec![];
        let mut parent_helper = [UVec3::ZERO; 8];

        plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
            parent_helper[p.level] = *p.position;

            seq.push((*p.parent_idx, (p.level), p.node_idx));

            // if p.level < 4 {
            //     dbg!(p.level);
            //     assert_eq!(
            //         &plat[0].get_node_idx_gpu(
            //             parent_helper[p.level + 1],
            //             p.level + 1,
            //             Some(p.entry as usize)
            //         ),
            //         p.parent_idx
            //     )
            // }
        });

        // assert_eq!(seq, alloc::vec![])
    }

    #[test]
    fn traverse_layer_full() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        // 0 0 0
        plat[0].set((0, 12, 8).into(), 22);
        plat[0].set((1, 12, 8).into(), 32);
        plat[0].set((2, 12, 8).into(), 52);
        plat[0].set((5, 12, 4).into(), 12);
        plat[0].set((1, 2, 11).into(), 2);

        // 1 1 0
        plat[0].set((34, 34, 8).into(), 32);
        plat[0].set((35, 32, 8).into(), 22);
        plat[0].set((50, 50, 4).into(), 12);
        plat[0].set((55, 60, 4).into(), 2);

        // 1 0 1
        plat[0].set((56, 2, 60).into(), 22);
        plat[0].set((57, 2, 60).into(), 32);
        plat[0].set((58, 2, 60).into(), 52);
        plat[0].set((59, 2, 60).into(), 12);
        plat[0].set((60, 2, 60).into(), 11);

        // 0 0 1
        plat[0].set((2, 16, 60).into(), 52);

        let mut seq = alloc::vec![];

        plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
            if p.level == 0 {
                seq.push(*p.position);
            }
        });

        assert_eq!(
            &seq,
            &alloc::vec![
                uvec3(0, 12, 8),
                uvec3(1, 12, 8),
                uvec3(2, 12, 8),
                uvec3(5, 12, 4),
                uvec3(1, 2, 11),
                //
                uvec3(34, 34, 8),
                uvec3(35, 32, 8),
                uvec3(50, 50, 4),
                uvec3(55, 60, 4),
                //
                uvec3(2, 16, 60),
                //
                uvec3(56, 2, 60),
                uvec3(57, 2, 60),
                uvec3(58, 2, 60),
                uvec3(59, 2, 60),
                uvec3(60, 2, 60)
            ]
        );
    }

    #[test]
    fn traverse_levels_and_check_ids() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        plat[0].set((0, 12, 8).into(), 22);
        plat[0].set((1, 12, 8).into(), 32);
        plat[0].set((2, 12, 8).into(), 52);
        plat[0].set((5, 12, 4).into(), 12);
        plat[0].set((1, 2, 11).into(), 2);

        let mut seq = alloc::vec![];

        plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
            seq.push(p.entry);
        });

        assert_eq!(
            &seq,
            &alloc::vec![
                0, 0, 22, 22, 22, 22, 22, 32, 32, 32, 32, 32, 52, 52, 52, 52, 52, 12, 12, 12, 12,
                12, 2, 2, 2, 2, 2, 0, 0, 0
            ]
        );

        plat.traverse(super::EntryOpts::All, LayerOpts::All, &mut |p| {
            // dbg!(p.level);
            if p.level != 4 {
                assert_eq!(
                    plat[0]
                        .get_node(*p.position, p.level, Some(p.entry as usize))
                        .voxel_id,
                    p.entry as usize
                )
            }
        });
    }

    #[test]
    fn traverse_check_path() {
        quick_raw_plat!(plat, depth 6, len 10);

        plat[0].set((7, 20, 5).into(), 52);

        let mut seq = alloc::vec![];

        let mut invocation_amount = 0;

        plat[0].traverse(0, 2, UVec3::ZERO, false, 6, &mut |p| {
            seq.push((*p.parent_idx, p.node_idx, plat[0][p.node_idx]));
            invocation_amount += 1;
        });
        dbg!(&plat[0]);
        assert_eq!(&seq, &alloc::vec![]);
        assert_eq!(invocation_amount, 7);
    }

    #[test]
    fn test_traverse_region_positions() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        // 0 0 0
        plat[0].set((0, 12, 8).into(), 22);
        plat[0].set((1, 12, 8).into(), 32);
        plat[0].set((2, 12, 8).into(), 52);
        plat[0].set((5, 12, 4).into(), 12);
        plat[0].set((1, 2, 11).into(), 2);

        // 1 1 0
        plat[0].set((34, 34, 8).into(), 32);
        plat[0].set((35, 32, 8).into(), 22);
        plat[0].set((50, 50, 4).into(), 12);
        plat[0].set((55, 60, 4).into(), 2);

        // 1 0 1
        plat[0].set((56, 2, 60).into(), 22);
        plat[0].set((57, 2, 60).into(), 32);
        plat[0].set((58, 2, 60).into(), 52);
        plat[0].set((59, 2, 60).into(), 12);
        plat[0].set((60, 2, 60).into(), 12);

        // 0 0 1
        plat[0].set((2, 16, 60).into(), 52);

        let mut seq = alloc::vec![];

        plat.traverse_region(
            uvec3(0, 0, 0),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    seq.push(*p.position);
                }
            },
        );

        assert_eq!(
            &seq,
            &alloc::vec![
                uvec3(0, 12, 8),
                uvec3(1, 12, 8),
                uvec3(2, 12, 8),
                uvec3(5, 12, 4),
                uvec3(1, 2, 11)
            ]
        );

        let mut seq = alloc::vec![];

        plat.traverse_region(
            uvec3(1, 1, 0),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    seq.push(*p.position);
                }
            },
        );

        assert_eq!(
            &seq,
            &alloc::vec![
                uvec3(2, 2, 8),
                uvec3(3, 0, 8),
                uvec3(18, 18, 4),
                uvec3(23, 28, 4),
            ]
        );

        let mut seq = alloc::vec![];

        plat.traverse_region(
            uvec3(1, 0, 1),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    seq.push(*p.position);
                }
            },
        );

        assert_eq!(
            &seq,
            &alloc::vec![
                uvec3(24, 2, 28),
                uvec3(25, 2, 28),
                uvec3(26, 2, 28),
                uvec3(27, 2, 28),
                uvec3(28, 2, 28)
            ]
        );

        let mut seq = alloc::vec![];

        plat.traverse_region(
            uvec3(0, 0, 1),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    seq.push(*p.position);
                }
            },
        );

        assert_eq!(&seq, &alloc::vec![uvec3(2, 16, 28)]);
    }

    #[test]
    fn partial_traverse_region() {
        let mut base = (Box::new([Node::default(); 23_000]), [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            7,
            5,
            5,
            (&mut *base.0, &mut base.1),
            (&mut *tmp.0, &mut tmp.1),
            (&mut *schem.0, &mut schem.1),
            (&mut *canvas.0, &mut canvas.1),
        );

        let mut rng = rand::thread_rng();

        let mtx: [[[u16; 16]; 16]; 16] = rng.gen();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let voxel_id = mtx[x][y][z] as u32 + 1;
                    plat[0].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
                }
            }
        }
        //let mut seq = vec![];

        plat.traverse_region(
            UVec3::ZERO,
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize][p.position.y as usize][p.position.z as usize]
                            as u32
                            + 1
                    );
                }
            },
        );
    }

    #[test]
    fn deep_traverse_region_full() {
        quick_raw_plat!(plat, depth 7, len 100_000_000);

        let mtx = gen_rand_mtx::<128>(50);

        for x in 0..128 {
            for y in 0..128 {
                for z in 0..128 {
                    let voxel_id = mtx[x][y][z];
                    plat[0].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
                }
            }
        }
        //let mut seq = vec![];

        plat.traverse_region(
            UVec3::ZERO,
            6,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize][p.position.y as usize][p.position.z as usize]
                    );

                    assert_eq!(p.entry, plat.get_voxel(*p.position).voxel_id as u32);
                }
            },
        );

        plat.traverse_region(
            uvec3(0, 1, 0),
            6,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize][p.position.y as usize + 64]
                            [p.position.z as usize]
                    );

                    assert_eq!(
                        p.entry,
                        plat.get_voxel(*p.position + uvec3(0, 1, 0) * 64).voxel_id as u32
                    );
                }
            },
        );

        plat.traverse_region(
            uvec3(1, 1, 1),
            6,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize + 64][p.position.y as usize + 64]
                            [p.position.z as usize + 64]
                    );

                    assert_eq!(
                        p.entry,
                        plat.get_voxel(*p.position + uvec3(1, 1, 1) * 64).voxel_id as u32
                    );
                }
            },
        );

        plat.traverse_region(
            uvec3(1, 1, 1),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize + 32][p.position.y as usize + 32]
                            [p.position.z as usize + 32]
                    );

                    assert_eq!(
                        p.entry,
                        plat.get_voxel(*p.position + uvec3(1, 1, 1) * 32).voxel_id as u32
                    );
                }
            },
        );

        plat.traverse_region(
            uvec3(2, 2, 2),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize + 64][p.position.y as usize + 64]
                            [p.position.z as usize + 64]
                    );

                    assert_eq!(
                        p.entry,
                        plat.get_voxel(*p.position + uvec3(2, 2, 2) * 32).voxel_id as u32
                    );
                }
            },
        );
        plat.traverse_region(
            uvec3(3, 2, 1),
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |p| {
                if p.level == 0 {
                    assert_eq!(
                        p.entry,
                        mtx[p.position.x as usize + 64 + 32][p.position.y as usize + 64]
                            [p.position.z as usize + 32]
                    );

                    assert_eq!(
                        p.entry,
                        plat.get_voxel(*p.position + uvec3(3, 2, 1) * 32).voxel_id as u32
                    );
                }
            },
        );
    }

    #[test]
    fn test_gen_rand_mtx() {
        let _ = gen_rand_mtx::<2>(50);
        let _ = gen_rand_mtx::<4>(0);
        let _ = gen_rand_mtx::<8>(100);
        let _ = gen_rand_mtx::<16>(50);
        let _ = gen_rand_mtx::<32>(50);
        let _ = gen_rand_mtx::<64>(50);
        let _ = gen_rand_mtx::<128>(50);
        let _ = gen_rand_mtx::<256>(50);
    }

    fn gen_rand_mtx<const SIZE: usize>(empty_probability: u8) -> Box<Vec<Vec<Vec<u32>>>> {
        let mut rng = rand::thread_rng();
        let mut mtx = Box::new(alloc::vec![alloc::vec![alloc::vec![0; SIZE]; SIZE]; SIZE]);

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if !rng.gen_ratio(empty_probability as u32, 100) {
                        let voxel_id: u16 = rng.gen();
                        // To prevent 0
                        mtx[x][y][z] = voxel_id as u32 + 1;
                    }
                }
            }
        }
        mtx
    }
    // #[test]
    // fn test_drop_tree() {
    //     todo!()
    // }

    // #[test]
    // fn check_parent_nodes() {
    //     let mut base = ([Node::default(); 128], [0; 10]);
    //     let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
    //     let mut plat = RawPlat::new(
    //         5,
    //         5,
    //         5,
    //         (&mut base.0, &mut base.1),
    //         (&mut tmp.0, &mut tmp.1),
    //         (&mut schem.0, &mut schem.1),
    //         (&mut canvas.0, &mut canvas.1),
    //     );
    //     // Base
    //     plat[Base].set(uvec3(7, 20, 5), 1);

    //     let mut seq = vec![];

    //     plat[Base].traverse(
    //         1,
    //         plat[Base].entries[1],
    //         UVec3::ZERO,
    //         true,
    //         plat.depth,
    //         &mut |props| {
    //             seq.push(props.parent_idx.clone());
    //         },
    //     );

    //     // println!("{seq:?}");

    //     //let addr = NodeAddr::from_position(uvec3(7, 20, 5), 5);

    //     // // let mut right
    //     // for level in (0..=5).rev() {
    //     //     plat.base[addr.get_idx(level)];
    //     // }
    //     todo!()
    //     // assert_eq!(seq, vec![]);
    // }

    #[test]
    fn traverse() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        // Base
        plat[Base].set(uvec3(14, 14, 14), 1);
        plat[Base].set(uvec3(0, 0, 0), 2);
        plat[Base].set(uvec3(5, 15, 5), 3);
        plat[Base].set(uvec3(0, 10, 0), 1);

        // Canvas
        plat[Canvas].set(uvec3(15, 15, 15), 1);
        plat[Canvas].set(uvec3(0, 0, 0), 2);

        let mut seq = alloc::vec![];

        plat.traverse(LayerOpts::All, EntryOpts::All, &mut |props| {
            if props.level == 0 {
                seq.push(props.position.clone());
            }
        });

        // println!("{seq:?}");

        assert_eq!(
            seq,
            [
                uvec3(0, 10, 0),
                uvec3(14, 14, 14),
                uvec3(0, 0, 0),
                uvec3(5, 15, 5),
                uvec3(15, 15, 15),
                uvec3(0, 0, 0)
            ]
        );
    }

    #[test]
    fn traverse_layer_single() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        // Base
        plat[Base].set(uvec3(7, 20, 5), 1);

        let mut seq = alloc::vec![];

        plat[Base].traverse(1, 2, UVec3::ZERO, true, plat.depth, &mut |props| {
            if props.level == 0 {
                seq.push(props.position.clone());
            }
        });

        // println!("{seq:?}");

        assert_eq!(seq, [uvec3(7, 20, 5)]);
    }

    #[test]
    fn traverse_layer() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        // Base
        plat[Base].set(uvec3(14, 14, 14), 1);
        plat[Base].set(uvec3(0, 0, 0), 1);
        plat[Base].set(uvec3(5, 15, 5), 1);
        plat[Base].set(uvec3(0, 10, 0), 1);
        plat[Base].set(uvec3(15, 15, 15), 1);

        let mut seq = alloc::vec![];

        plat[Base].traverse(1, 2, UVec3::ZERO, true, plat.depth, &mut |props| {
            if props.level == 0 {
                seq.push(props.position.clone());
            }
        });

        // println!("{seq:?}");

        assert_eq!(
            seq,
            [
                uvec3(0, 0, 0),
                uvec3(0, 10, 0),
                uvec3(5, 15, 5),
                uvec3(14, 14, 14),
                uvec3(15, 15, 15)
            ]
        );
    }
}
