use glam::{uvec3, UVec3};

use super::graph::{GBranch, GNode, Graph};

impl Graph {
    fn get(&mut self, mut pos: UVec3, on_level: u8) -> Option<bool> {
        todo!()
    }
    pub fn at(&self, level: u8, mut position: UVec3) -> Option<usize> {
        let node = self.nodes[0].get_branch().unwrap();

        let mut lvl = node.level();

        let mut size = self.size();

        let mut child_index;

        let mut idx = None;

        while lvl > level {
            child_index = GBranch::get_child_index(position, lvl - 1);

            if child_index != 0 {
                let child_idx = node.children[child_index];
                idx = Some(child_idx as usize);
            } else {
                return None;
            }

            {
                size /= 2;
                position %= size;
                lvl -= 1;
            }
        }

        idx
    }
}

#[test]
fn test_getters() {
    let mut graph = Graph::new(5);
    graph.set(uvec3(0, 0, 0), true);
    graph.set(uvec3(0, 5, 0), true);
    graph.set(uvec3(1, 1, 0), true);
    graph.set(uvec3(0, 0, 0), true);

    graph.at(1, uvec3(0, 0, 0)).unwrap();
    // assert_eq!(graph.at(0, uvec3(0, 5, 0)), true);
    // assert_eq!(graph.at(0, uvec3(1, 1, 0)), true);
    // assert_eq!(graph.at(0, uvec3(2, 0, 6)), true);
    // assert_eq!(graph.at(0, uvec3(1, 4, 9)), true);
    assert!(false);
}
