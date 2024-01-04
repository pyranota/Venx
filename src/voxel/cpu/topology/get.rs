use glam::{uvec3, UVec3};

use crate::voxel::{cpu::voxel::Voxel, segment::Segment};

use super::graph::{Branch, Graph, Idx};

impl Graph {
    // pub fn is_at(&self, level: u8, mut position: UVec3) -> bool {
    //     //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
    //     let mut current_level = self.depth as u8;
    //     let mut size = self.size();
    //     let mut found_idx = false;
    //     let mut idx = 0;

    //     while current_level > level {
    //         let child_index = GBranch::get_child_index(position, current_level - 1);
    //         let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
    //         if child_id != 0 {
    //             idx = child_id as usize;
    //             found_idx = true;
    //         } else {
    //             return false;
    //         }
    //         {
    //             size /= 2;
    //             position %= size;
    //             current_level -= 1;
    //         }
    //     }

    //     found_idx
    // }
    // pub fn get_attr_position(&self, level: u8, mut position: UVec3) -> Option<AttrPosition> {
    //     //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
    //     let mut current_level = self.depth as u8;
    //     let mut size = self.size();
    //     let mut idx = 0;
    //     let mut attr_position = 0;

    //     while current_level > level {
    //         let child_index = GBranch::get_child_index(position, current_level - 1);
    //         attr_position += self.count_children(idx, child_index);
    //         let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
    //         if child_id != 0 {
    //             idx = child_id as usize;
    //         } else {
    //             return None;
    //         }
    //         {
    //             size /= 2;
    //             position %= size;
    //             current_level -= 1;
    //         }
    //     }
    //     let child_index = GBranch::get_child_index(position, current_level);
    //     attr_position += self.count_children(idx, child_index);

    //     Some(attr_position)
    // }
    pub fn get_node_cached(&self, level: usize, path: &Vec<usize>) -> Option<Idx> {
        todo!()
        // //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        // let mut current_level = self.depth as usize;
        // let mut size = self.size();
        // let mut found_idx = None;

        // let mut idx = 0;

        // while current_level > level {
        //     let child_index = path[current_level];

        //     let child_id = self.nodes[idx].children[child_index];
        //     if child_id != 0 {
        //         idx = child_id as usize;
        //         found_idx = Some(child_id as usize);
        //     } else {
        //         return None;
        //     }
        //     {
        //         current_level -= 1;
        //     }
        // }

        // if let Some(idx) = found_idx {
        //     return Some(idx);
        // }
        // None
    }
    pub fn get_node(&self, level: u8, mut position: UVec3, entry: Idx) -> Option<Idx> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        //todo!()
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut found_idx = None;

        let mut idx = entry; // 1;
                             // dbg!("Enter");
        while current_level > level {
            let child_index = Branch::get_child_index(position, current_level - 1);
            // dbg!(child_index);
            // panic!();
            let child_id = self.levels[current_level as usize][idx].children[child_index];

            // dbg!(child_id);
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
        let child_index = Branch::get_child_index(position, current_level);
        if let Some(idx) = found_idx {
            return Some(idx);
        }
        None
    }

    // pub fn get_node(&self, level: u8, mut position: UVec3) -> Option<Idx> {
    //     //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
    //     //todo!()
    //     let mut current_level = self.depth as u8;
    //     let mut size = self.size();
    //     let mut found_idx = None;

    //     let mut idx = 0;
    //     let mut count = 0;

    //     while current_level > level {
    //         let child_index = GBranch::get_child_index(position, current_level - 1);

    //         let child_id = self.levels[level as usize][idx].children[child_index];
    //         if child_id != 0 {
    //             idx = child_id as usize;
    //             found_idx = Some(child_id as usize);
    //         } else {
    //             return None;
    //         }
    //         {
    //             size /= 2;
    //             position %= size;
    //             current_level -= 1;
    //         }
    //     }
    //     let child_index = GBranch::get_child_index(position, current_level);
    //     if let Some(idx) = found_idx {
    //         return Some(idx);
    //     }
    //     None
    // }
    // pub fn get_node_untyped(&self, level: u8, mut position: UVec3) -> Option<AttrPosition> {
    //     //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
    //     let mut current_level = self.depth as u8;
    //     let mut size = self.size();
    //     let mut found_idx = None;
    //     let mut idx = 0;
    //     let mut count = 0;

    //     while current_level > level {
    //         let child_index = GBranch::get_child_index(position, current_level - 1);
    //         count += self.count_children(idx, child_index);
    //         let child_id = self.nodes[idx].get_branch().unwrap().children[child_index];
    //         if child_id != 0 {
    //             idx = child_id as usize;
    //             found_idx = Some(0);
    //         } else {
    //             return None;
    //         }
    //         {
    //             size /= 2;
    //             position %= size;
    //             current_level -= 1;
    //         }
    //     }

    //     todo!()
    // }
}

#[test]
fn test_untyped() {
    let mut graph = Graph::new(5);
    graph.set(uvec3(0, 0, 0), 1);
    graph.set(uvec3(0, 5, 0), 1);
    graph.set(uvec3(1, 1, 0), 1);
    graph.set(uvec3(0, 0, 0), 1);

    // dbg!(&graph);

    // graph.get(0, uvec3(0, 0, 0)).unwrap();
    assert!(graph.get_node(0, uvec3(0, 0, 0), 1).is_some());
    assert!(graph.get_node(1, uvec3(0, 5, 0), 1).is_some());
}
// #[test]
// fn test_typed() {
//     let mut vx = Voxel::new(4, 2, 3);
//     let mut segment = Segment::new(3);
//     segment.set((0, 0, 0), 55);
//     segment.set((0, 1, 1), 12);
//     vx.insert_segment(segment, (0, 0, 0).into());

//     dbg!(vx.get(0, (0, 0, 0).into()));
//     dbg!(vx.get(0, (0, 1, 1).into()));

//     dbg!(vx.attribute);
//     // assert_eq!(graph.at(0, uvec3(0, 5, 0)), true);
//     // assert_eq!(graph.at(0, uvec3(1, 1, 0)), true);
//     // assert_eq!(graph.at(0, uvec3(2, 0, 6)), true);
//     // assert_eq!(graph.at(0, uvec3(1, 4, 9)), true);
// }
