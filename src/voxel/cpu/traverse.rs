use glam::{uvec3, UVec3};

use crate::voxel::cpu::{
    topology::{
        graph::{Branch, Graph},
        level::GLevel,
    },
    utils::lvl_to_size::lvl_to_size,
};

use super::{topology::graph::Idx, voxel::Voxel};

pub enum TrProps<'a> {
    /// Leaf is always on 0 level
    Leaf {
        position: &'a UVec3,
        /// If parent is 0 it means, there is no parent
        parent: Idx,
        node: Idx,
    },
    Branch {
        parent: Idx,
        node: Idx,
        children: &'a [u32; 8],
        position: &'a UVec3,
        level: u8,
    },
}

pub struct TrPropsMut<'a> {
    pub position: &'a UVec3,
    pub parent: &'a mut Branch,
    pub node: &'a mut Branch,
    pub level: u8,
}

impl Graph {
    //     // todo: Move to graph class
    //     /// Traversing each node and calling given closure with args: Node, Index, Position
    //     pub fn traverse<F>(&self, mut f: F)
    //     where
    //         F: FnMut(&GBranch, usize, UVec3) -> bool,
    //     {
    //         visit_node(self, 0, UVec3::ZERO, &mut f);

    //         fn visit_node<F>(vx: &Voxel, idx: usize, node_position: UVec3, f: &mut F)
    //         where
    //             F: FnMut(&GBranch, usize, UVec3) -> bool,
    //         {
    //             let branch = vx.topology.nodes[idx].get_branch().unwrap();

    //             if !f(branch, idx, node_position) {
    //                 return;
    //             }
    //             let size = branch.size() / 2;
    //             for (i, child_id) in (branch.children).into_iter().enumerate() {
    //                 if child_id != 0 {
    //                     let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;

    //                     visit_node(vx, child_id as usize, child_pos, f);
    //                 }
    //             }
    //         }
    //     }

    /// Traversing each node and calling given closure with args: Node, Index, Position, level
    /// Return false in closure to drop traversing of subtree
    // pub fn traverse_from_unpositioned_mut<F>(&self, idx: usize, level: u8, mut f: F)
    // where
    //     F: FnMut(TrPropsMut) -> bool,
    // {
    //     const POSITION: UVec3 = uvec3(0, 0, 0);
    //     visit_node(self, idx, 0, level, &mut f);

    //     fn visit_node<F>(graph: &Graph, idx: Idx, parent: Idx, level: u8, f: &mut F)
    //     where
    //         F: FnMut(TrPropsMut) -> bool,
    //     {
    //         let node = &graph.levels[level as usize][idx];

    //         // Compose props
    //         let props = if level == 0 {
    //             TrProps::Leaf {
    //                 parent,
    //                 position: &POSITION,
    //                 idx: idx,
    //             }
    //         } else {
    //             TrProps::Branch {
    //                 parent,
    //                 children: &node.children,
    //                 position: &POSITION,
    //                 level,
    //             }
    //         };

    //         let props = TrPropsMut {
    //             position: &POSITION,
    //             parent:
    //         };

    //         // Call back the user function.
    //         if !f(props) {
    //             return;
    //         }

    //         if level == 0 {
    //             // There is not anything left, lets return
    //             return;
    //         }

    //         for child_idx in (node.children).iter() {
    //             // Remember, cant index 0
    //             if child_idx != &0 {
    //                 // Global voxel position in 3d space coordinates.

    //                 visit_node(graph, *child_idx as usize, idx, level - 1, f);
    //             }
    //         }
    //     }
    // }
    /// Traversing each node and calling given closure with args: Node, Index, Position, level
    /// Return false in closure to drop traversing of subtree
    pub fn traverse_from_unpositioned<F>(levels: &Vec<GLevel>, idx: usize, level: u8, mut f: F)
    where
        F: FnMut(TrProps) -> bool,
    {
        const POSITION: UVec3 = uvec3(0, 0, 0);
        visit_node(levels, idx, 0, level, &mut f);

        fn visit_node<F>(levels: &Vec<GLevel>, idx: Idx, parent: Idx, level: u8, f: &mut F)
        where
            F: FnMut(TrProps) -> bool,
        {
            let node = &levels[level as usize][idx];

            // Compose props
            let props = if level == 0 {
                TrProps::Leaf {
                    parent,
                    position: &POSITION,
                    node: idx,
                }
            } else {
                TrProps::Branch {
                    parent,
                    children: &node.children,
                    position: &POSITION,
                    level,
                    node: idx,
                }
            };

            // Call back the user function.
            if !f(props) {
                return;
            }

            if level == 0 {
                // There is not anything left, lets return
                return;
            }

            for child_idx in (node.children).iter() {
                // Remember, cant index 0
                if child_idx != &0 {
                    // Global voxel position in 3d space coordinates.

                    visit_node(levels, *child_idx as usize, idx, level - 1, f);
                }
            }
        }
    }
    /// Traversing each node and calling given closure with args: Node, Index, Position, level
    /// Return false in closure to drop traversing of subtree
    pub fn traverse_from<F>(&self, idx: usize, node_position: UVec3, level: u8, mut f: F)
    where
        F: FnMut(TrProps) -> bool,
    {
        visit_node(self, idx, 0, node_position, level, &mut f);

        fn visit_node<F>(
            graph: &Graph,
            idx: Idx,
            parent: Idx,
            node_position: UVec3,
            level: u8,
            f: &mut F,
        ) where
            F: FnMut(TrProps) -> bool,
        {
            //   dbg!(level);
            let node = &graph.levels[level as usize][idx];

            // Compose props
            let props = if level == 0 {
                TrProps::Leaf {
                    parent,
                    position: &node_position,
                    node: idx,
                }
            } else {
                TrProps::Branch {
                    parent,
                    children: &node.children,
                    position: &node_position,
                    level,
                    node: idx,
                }
            };

            // Call back the user function.
            if !f(props) {
                return;
            }

            if level == 0 {
                // There is not anything left, lets return
                return;
            }

            let size = lvl_to_size(level) / 2;

            for (i, child_idx) in (node.children).into_iter().enumerate() {
                // Remember, cant index 0
                if child_idx != 0 {
                    // Global voxel position in 3d space coordinates.
                    let child_pos = Branch::get_child_position(i as u32) * (size) + node_position;

                    visit_node(graph, child_idx as usize, idx, child_pos, level - 1, f);
                }
            }
        }
    }
    //     /// Traversing each node and calling given closure with args: Node, Index, Position
    //     pub fn traverse_untyped_from<F>(&self, idx: usize, node_position: UVec3, mut f: F)
    //     where
    //         F: FnMut(&GBranch, usize, UVec3) -> bool,
    //     {
    //         visit_node(self, idx, node_position, &mut f);

    //         fn visit_node<F>(vx: &Voxel, idx: usize, node_position: UVec3, f: &mut F)
    //         where
    //             F: FnMut(&GBranch, usize, UVec3) -> bool,
    //         {
    //             let node = vx.topology.nodes[idx].get_branch().unwrap();

    //             if !f(node, idx, node_position) {
    //                 return;
    //             }
    //             // ?
    //             let size = node.size() / 2;

    //             for (i, child_idx) in (node.children).into_iter().enumerate() {
    //                 if child_idx != 0 {
    //                     let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;

    //                     visit_node(vx, child_idx as usize, child_pos, f);
    //                 }
    //             }
    //         }
    //     }
}

#[test]
fn test_traverse() {
    let mut voxel = Voxel::new(3, 0, 0);

    voxel.layers[0].graph.set(uvec3(0, 0, 0), 1);
    voxel.layers[0].graph.set(uvec3(0, 2, 0), 1);
    voxel.layers[0].graph.set(uvec3(1, 2, 4), 1);

    // voxel.layers[0]
    //     .graph
    //     .traverse_from(0, uvec3(0, 0, 0), |branch, idx, pos| {
    //         dbg!(branch, pos, idx);
    //         true
    //     });

    // voxel.traverse(|branch, idx, pos| {
    //     dbg!(branch, pos, idx);
    //     true
    // });
}
