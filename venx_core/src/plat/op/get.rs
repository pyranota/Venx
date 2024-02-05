use spirv_std::glam::{UVec3, Vec3};

use crate::plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat};

use super::{EntryOpts, LayerOpts};

impl RawPlat {
    /// If Entry is Entry::All, than it will return the most valuable (by voxel-collection) block
    /// Same goes for Layer, if it is Layer::All, it will return the most higher layer
    pub fn get_node(
        &self,
        position: UVec3,
        level: u8,
        entry: EntryOpts,
        layer: LayerOpts,
    ) -> Option<usize> {
        // TODO make binary (take u64 and divide by 3 bits)
        // Small optimization
        // With this we should not calculate children indices each run.
        //let path = self.find_path(position.as_uvec3(), level);
        //todo!()
        //let path = [];

        self.opts(
            Some(position),
            layer,
            entry,
            false,
            &mut |_plat, layer, entry| {
                if let Some(entry) =
                    self.get_node_direct(position, level, layer as usize, entry as usize)
                {
                    return Some(entry);
                }
                None
            },
        )
        // self.get_node_pathed(position, level, entry, layer, &path)
    }

    fn find_path<'a>(&self, mut position: UVec3, to_level: u8) -> &'a [usize] {
        let mut path = [0; 20];
        let mut current_level = self.depth as u8;
        let mut size = self.size();

        while current_level > to_level {
            let child_index = Node::get_child_index(position, current_level - 1);
            path[current_level as usize] = child_index;

            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }
        todo!()
        // &path
    }

    fn get_node_cached(
        &self,
        mut position: UVec3,
        level: u8,
        layer: usize,
        entry: usize,
    ) -> Option<usize> {
        todo!()
    }

    fn get_node_direct(
        &self,
        mut position: UVec3,
        level: u8,
        layer: usize,
        entry: usize,
    ) -> Option<usize> {
        //let child_pos = GBranch::get_child_position(i as u32) * (size) + node_position;
        //todo!()
        let mut current_level = self.depth as u8;

        let mut size = self.size();
        let mut found_idx = None;

        let mut idx = entry; // 1;

        while current_level > level {
            let child_index = Node::get_child_index(position, current_level - 1);
            // dbg!(child_index);
            // panic!();
            let child_id = self[layer][idx].children[child_index];

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
        let child_index = Node::get_child_index(position, current_level);
        if let Some(idx) = found_idx {
            return Some(idx);
        }
        None
    }

    pub fn get_voxel(&self, position: UVec3, layer: LayerOpts) {
        self.get_node(position, 0, EntryOpts::All, layer);
    }

    /// Is there a voxel or not at given position
    /// Slowest operation you should avoid it as much as possible
    pub fn at(&self, position: Vec3, level: u8, entry: EntryOpts, layer: LayerOpts) -> bool {
        // Small optimization
        // With this we should not calculate children indices each run.
        // let path = self.find_path(position: Vec3, 0);

        // self.get_node_pathed(..).is_some()
        todo!()
    }

    // solid_at -> solid_at_specific. Solid at has no more entry and layer
    pub fn solid_at(&self, position: Vec3, level: u8, entry: EntryOpts, layer: LayerOpts) -> bool {
        todo!()
    }
}
