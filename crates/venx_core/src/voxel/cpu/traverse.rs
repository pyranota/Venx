use glam::{uvec3, UVec3};

use crate::voxel::cpu::topology::graph::Graph;

use super::{topology::graph::GBranch, voxel::Voxel};

// impl Voxel {
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

//     /// Traversing each node and calling given closure with args: Node, Index, Position
//     pub fn traverse_from<F>(
//         &self,
//         idx: usize,
//         node_position: UVec3,
//         attribute_position: u32,
//         mut f: F,
//     ) where
//         F: FnMut(&GBranch, usize, UVec3, i32) -> bool,
//     {
//         visit_node(self, attribute_position, idx, node_position, &mut f);

//         fn visit_node<F>(
//             vx: &Voxel,
//             mut attr_position: u32,
//             idx: usize,
//             node_position: UVec3,
//             f: &mut F,
//         ) where
//             F: FnMut(&GBranch, usize, UVec3, i32) -> bool,
//         {
//             let node = vx.topology.nodes[idx].get_branch().unwrap();

//             let block = vx.attribute.get(attr_position).unwrap();

//             if !f(node, idx, node_position, block.0) {
//                 return;
//             }

//             if node.level() == 0 {
//                 return;
//             }

//             let size = node.size() / 2;

//             for (i, child_idx) in (node.children).into_iter().enumerate() {
//                 if child_idx != 0 {
//                     let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;

//                     visit_node(vx, attr_position, child_idx as usize, child_pos, f);
//                     attr_position += vx.topology.nodes[child_idx as usize]
//                         .get_branch()
//                         .unwrap()
//                         .attr_count;
//                 }
//             }
//         }
//     }
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
// }

// #[test]
// fn test_traverse() {
//     let mut voxel = Voxel::new(3, 0, 0);

//     voxel.topology.set(uvec3(0, 0, 0), true);
//     voxel.topology.set(uvec3(0, 2, 0), true);
//     voxel.topology.set(uvec3(1, 2, 4), true);

//     voxel.traverse_untyped_from(0, uvec3(0, 0, 0), |branch, idx, pos| {
//         dbg!(branch, pos, idx);
//         true
//     });

//     voxel.traverse(|branch, idx, pos| {
//         dbg!(branch, pos, idx);
//         true
//     });
// }
