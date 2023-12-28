use glam::UVec3;

use crate::voxel::{
    cpu::{
        topology::graph::{GBranch, Idx},
        utils::lvl_to_size::lvl_to_size,
        voxel::Voxel,
    },
    interfaces::layer::LayerInterface,
};

use super::VXLayer;

impl VXLayer {
    /// Returns block type and its id in slice (if found)
    pub fn get_voxel(&self, position: UVec3) -> Option<(usize, Idx)> {
        for (ty, slice) in &self.slices {
            if let Some(idx) = slice.graph.get_node(0, position) {
                return Some((*ty, idx));
            }
        }
        return None;
    }

    pub fn get_voxel_cached(&self, mut position: UVec3) -> Option<(usize, Idx)> {
        let mut path = vec![0; self.depth as usize + 1];

        let mut current_level = self.depth as u8;
        let mut size = lvl_to_size(self.depth);

        while current_level > 0 {
            let child_index = GBranch::get_child_index(position, current_level - 1);

            path[current_level as usize] = child_index;

            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        for (ty, slice) in &self.slices {
            if let Some(idx) = slice.graph.get_node_cached(0, &path) {
                return Some((*ty, idx));
            }
        }
        return None;
    }
}

#[test]
fn test_get() {
    let mut voxel = Voxel::new(5, 4, 5);
    voxel.set_voxel(0, (1, 2, 3).into(), 1);
    voxel.set_voxel(0, (2, 2, 2).into(), 2);
    voxel.set_voxel(0, (3, 2, 3).into(), 3);
    voxel.set_voxel(0, (3, 1, 3).into(), 4);

    assert!(voxel.layers[0].get_voxel((1, 2, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel((2, 2, 2).into()).is_some());
    assert!(voxel.layers[0].get_voxel((3, 2, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel((3, 1, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel((0, 1, 2).into()).is_none());

    // Test cached
    assert!(voxel.layers[0].get_voxel_cached((1, 2, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel_cached((2, 2, 2).into()).is_some());
    assert!(voxel.layers[0].get_voxel_cached((3, 2, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel_cached((3, 1, 3).into()).is_some());
    assert!(voxel.layers[0].get_voxel_cached((0, 1, 2).into()).is_none());
}
