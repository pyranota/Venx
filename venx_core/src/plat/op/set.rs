use spirv_std::glam::UVec3;

use crate::{
    plat::{layer::layer::Layer, node::Node},
    utils::l2s,
};

impl Layer {
    /// ty 0 is reserved for air and will remove voxel if there is any
    /// you can add any ty if there is no already created entry for it
    /// It will create one
    pub fn set(&mut self, mut pos: UVec3, entry: u32) {
        if entry == 0 {
            return;
        }
        // Identify starting point according to given entry
        let mut idx = self.entry(entry as usize);
        // dbg!(idx, entry);

        let mut size = l2s(self.depth);

        let mut level = self.depth;

        // If given position is out of bound
        if pos.y >= size || pos.x >= size || pos.z >= size {
            return;
        }

        while level > 1 {
            let child_index = Node::get_child_index(pos, level - 1);

            let branch = self[idx];

            let child_id = branch.children[child_index];

            if child_id == 0 {
                let new_child_id = self.allocate_node();
                self[idx].children[child_index] = new_child_id as u32;
                idx = new_child_id;
            } else {
                idx = self[idx].children[child_index] as usize;
            }

            {
                size /= 2;
                level -= 1;
                pos.x %= size;
                pos.y %= size;
                pos.z %= size;
            }
        }
        let child_index = Node::get_child_index(pos, 0);
        let branch = &mut self[idx];
        if entry != 0 {
            branch.children[child_index] = 1;
        } else {
            panic!("D:");
            todo!()
        }
    }
}
