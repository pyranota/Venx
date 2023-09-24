use glam::{uvec3, UVec3};

use super::graph::{GBranch, GNode, Graph};

impl Graph {
    pub fn set_untyped(&mut self, mut pos: UVec3, data: bool) {
        let mut size = self.size();
        let mut idx = 0;
        let mut level = self.depth();

        if pos.y >= size || pos.x >= size || pos.z >= size {
            return;
        }

        while level > 1 {
            let child_index = GBranch::get_child_index(pos, level - 1);

            if let Some(branch) = self.nodes[idx].get_branch() {
                let child_id = branch.children[child_index];

                if child_id == 0 {
                    let new_child_id = self.add_branch(GBranch::new(level - 1));
                    self.nodes[idx].get_branch_mut().unwrap().children[child_index] =
                        new_child_id as u32;
                    idx = new_child_id;
                } else {
                    idx = self.nodes[idx].get_branch().unwrap().children[child_index] as usize;
                }
            } else {
                panic!("It should not be here");
            }
            {
                size /= 2;
                level -= 1;
                pos.x %= size;
                pos.y %= size;
                pos.z %= size;
            }
        }
        let child_index = GBranch::get_child_index(pos, 0);

        self.nodes[idx].get_branch_mut().unwrap().children[child_index] = 1;
    }
    // TODO: move to graph decl module
    fn get_node_attr_count(&self, node_idx: usize) -> u32 {
        return if node_idx != 0 {
            self.nodes[node_idx].get_branch().unwrap().attr_count
        } else {
            0
        };
    }
    pub fn set(&mut self, mut pos: UVec3, data: bool) -> u32 {
        let mut size = self.size();
        let mut idx = 0;
        let mut level = self.depth();

        if pos.y >= size || pos.x >= size || pos.z >= size {
            return 0;
        }

        let mut global_counter = 0;

        while level > 1 {
            self.nodes[idx].get_branch_mut().unwrap().attr_count += 1;
            let child_index = GBranch::get_child_index(pos, level - 1);

            if let Some(branch) = self.nodes[idx].get_branch() {
                for i in 0..child_index {
                    global_counter += self.get_node_attr_count(branch.children[i] as usize);
                }

                let child_id = branch.children[child_index];

                if child_id == 0 {
                    let new_child_id = self.add_branch(GBranch::new(level - 1));
                    self.nodes[idx].get_branch_mut().unwrap().children[child_index] =
                        new_child_id as u32;
                    idx = new_child_id;
                } else {
                    idx = self.nodes[idx].get_branch().unwrap().children[child_index] as usize;
                }
            } else {
                panic!("It should not be here");
            }
            {
                size /= 2;
                level -= 1;
                pos.x %= size;
                pos.y %= size;
                pos.z %= size;
            }
        }
        let child_index = GBranch::get_child_index(pos, 0);
        let branch = self.nodes[idx].get_branch_mut().unwrap();
        branch.children[child_index] = 1;
        branch.attr_count += 1;
        for i in 0..child_index {
            let idx = branch.children[i];
            if idx == 1 {
                global_counter += 1;
            }
        }
        global_counter
    }
}

#[test]
fn set_voxel() {
    let mut graph = Graph::new(1);
    graph.set(uvec3(0, 0, 0), true);
    graph.set(uvec3(3, 3, 0), true);
    dbg!(graph);
}
