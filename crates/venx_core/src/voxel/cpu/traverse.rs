use glam::{uvec3, UVec3};

use crate::voxel::cpu::topology::graph::Graph;

use super::{topology::graph::GBranch, voxel::Voxel};

impl Voxel {
    /// Traversing each node and calling given closure with args: Node, Index, Position
    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        visit_node(self, 0, UVec3::ZERO, &mut f);

        fn visit_node<F>(vx: &Voxel, idx: usize, node_position: UVec3, f: &mut F)
        where
            F: FnMut(&GBranch, usize, UVec3) -> bool,
        {
            let branch = vx.topology.nodes[idx].get_branch().unwrap();

            if !f(branch, idx, node_position) {
                return;
            }
            let size = branch.size() / 2;
            for (i, child_id) in (branch.children).into_iter().enumerate() {
                if child_id != 0 {
                    let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;

                    visit_node(vx, child_id as usize, child_pos, f);
                }
            }
        }
    }

    /// Traversing each node and calling given closure with args: Node, Index, Position
    pub fn traverse_from<F>(&self, idx: usize, node_position: UVec3, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        visit_node(self, idx, node_position, &mut f);

        fn visit_node<F>(vx: &Voxel, idx: usize, node_position: UVec3, f: &mut F)
        where
            F: FnMut(&GBranch, usize, UVec3) -> bool,
        {
            let node = vx.topology.nodes[idx].get_branch().unwrap();

            if !f(node, idx, node_position) {
                return;
            }
            // ?
            let size = node.size() / 2;

            for (i, child_idx) in (node.children).into_iter().enumerate() {
                if child_idx != 0 {
                    let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;

                    visit_node(vx, child_idx as usize, child_pos, f);
                }
            }
        }
    }
}

#[test]
fn test_traverse() {
    let mut voxel = Voxel::new(3);

    voxel.topology.set(uvec3(0, 0, 0), true);
    voxel.topology.set(uvec3(0, 2, 0), true);
    voxel.topology.set(uvec3(1, 2, 4), true);

    voxel.traverse_from(0, uvec3(0, 0, 0), |branch, idx, pos| {
        dbg!(branch, pos, idx);
        true
    });

    voxel.traverse(|branch, idx, pos| {
        dbg!(branch, pos, idx);
        true
    });
}
