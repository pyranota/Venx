use glam::{uvec3, UVec3};

use crate::voxel::{
    cpu::{
        facade::{AttrPosition, Idx},
        voxel::Voxel,
    },
    segment::Segment,
    vx_trait::VoxelTrait,
};

use super::graph::{GBranch, GNode, Graph};

impl Graph {
    pub fn is_at(&self, level: u8, mut position: UVec3) -> bool {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut found_idx = false;
        let mut idx = 0;

        while current_level > level {
            let child_index = GBranch::get_child_index(position, current_level - 1);
            let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
            if child_id != 0 {
                idx = child_id as usize;
                found_idx = true;
            } else {
                return false;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        found_idx
    }
    pub fn get_attr_position(&self, level: u8, mut position: UVec3) -> Option<AttrPosition> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut idx = 0;
        let mut attr_position = 0;

        while current_level > level {
            let child_index = GBranch::get_child_index(position, current_level - 1);
            attr_position += self.count_children(idx, child_index);
            let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
            if child_id != 0 {
                idx = child_id as usize;
            } else {
                return None;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }
        let child_index = GBranch::get_child_index(position, current_level);
        attr_position += self.count_children(idx, child_index);

        Some(attr_position)
    }
    pub fn get_node(&self, level: u8, mut position: UVec3) -> Option<(Idx, AttrPosition)> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut found_idx = None;
        let mut attr_position = 0;

        let mut idx = 0;
        let mut count = 0;

        while current_level > level {
            let child_index = GBranch::get_child_index(position, current_level - 1);
            attr_position += self.count_children(idx, child_index);

            let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
            if child_id != 0 {
                idx = child_id as usize;
                found_idx = Some(child_id as usize);
            } else {
                return None;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }
        let child_index = GBranch::get_child_index(position, current_level);
        attr_position += self.count_children(idx, child_index);
        if let Some(idx) = found_idx {
            return Some((idx, attr_position));
        }
        None
    }
    pub fn get_node_untyped(&self, level: u8, mut position: UVec3) -> Option<AttrPosition> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut found_idx = None;
        let mut idx = 0;
        let mut count = 0;

        while current_level > level {
            let child_index = GBranch::get_child_index(position, current_level - 1);
            count += self.count_children(idx, child_index);
            let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
            if child_id != 0 {
                idx = child_id as usize;
                found_idx = Some(0);
            } else {
                return None;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        todo!()
    }
}

#[test]
fn test_untyped() {
    let mut graph = Graph::new(5);
    graph.set(uvec3(0, 0, 0), true);
    graph.set(uvec3(0, 5, 0), true);
    graph.set(uvec3(1, 1, 0), true);
    graph.set(uvec3(0, 0, 0), true);

    // graph.get(0, uvec3(0, 0, 0)).unwrap();
    dbg!(graph.get_node_untyped(0, uvec3(0, 0, 0)));
    dbg!(graph.get_node_untyped(1, uvec3(0, 5, 0)));
    // assert_eq!(graph.at(0, uvec3(0, 5, 0)), true);
    // assert_eq!(graph.at(0, uvec3(1, 1, 0)), true);
    // assert_eq!(graph.at(0, uvec3(2, 0, 6)), true);
    // assert_eq!(graph.at(0, uvec3(1, 4, 9)), true);
}
#[test]
fn test_typed() {
    let mut vx = Voxel::new(4, 2, 3);
    let mut segment = Segment::new(3);
    segment.set((0, 0, 0), 55);
    segment.set((0, 1, 1), 12);
    vx.insert_segment(segment, (0, 0, 0).into());

    dbg!(vx.get(0, (0, 0, 0).into()));
    dbg!(vx.get(0, (0, 1, 1).into()));

    dbg!(vx.attribute);
    // assert_eq!(graph.at(0, uvec3(0, 5, 0)), true);
    // assert_eq!(graph.at(0, uvec3(1, 1, 0)), true);
    // assert_eq!(graph.at(0, uvec3(2, 0, 6)), true);
    // assert_eq!(graph.at(0, uvec3(1, 4, 9)), true);
}
