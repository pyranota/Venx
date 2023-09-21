use glam::{uvec3, UVec3};

use super::graph::{GBranch, GNode, Graph};

impl Graph {
    pub fn get(&self, level: u8, mut position: UVec3) -> Option<usize> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        let mut current_level = self.depth as u8;
        let mut size = self.size();
        let mut found_idx = None;
        let mut idx = 0;

        while current_level > level {
            let child_index = GBranch::get_child_index(position, current_level - 1);
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

        found_idx
    }
}

#[test]
fn test_getters() {
    let mut graph = Graph::new(5);
    graph.set(uvec3(0, 0, 0), true);
    graph.set(uvec3(0, 5, 0), true);
    graph.set(uvec3(1, 1, 0), true);
    graph.set(uvec3(0, 0, 0), true);

    // graph.get(0, uvec3(0, 0, 0)).unwrap();
    dbg!(graph.get(0, uvec3(0, 0, 0)));
    dbg!(graph.get(1, uvec3(0, 5, 0)));
    // assert_eq!(graph.at(0, uvec3(0, 5, 0)), true);
    // assert_eq!(graph.at(0, uvec3(1, 1, 0)), true);
    // assert_eq!(graph.at(0, uvec3(2, 0, 6)), true);
    // assert_eq!(graph.at(0, uvec3(1, 4, 9)), true);
}
