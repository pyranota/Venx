use glam::UVec3;

use crate::voxel::{
    cpu::{utils::lvl_to_size::lvl_to_size, voxel::Voxel},
    interfaces::layer::LayerInterface,
};

impl LayerInterface for Voxel {
    fn set_voxel(&mut self, layer: usize, position: UVec3, ty: usize) {
        self.layers[layer].graph.set(position, ty as u32);
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn set_segment(
        &mut self,
        layer: usize,
        segment: crate::voxel::segment::Segment,
        position: UVec3,
    ) {
        // TODO! Check if there is no segment already
        // Print warning if there were blocks.
        segment.iter(|coord, ty| {
            self.set_voxel(
                layer,
                coord + position * lvl_to_size(self.segment_level),
                ty as usize,
            );
        });
    }

    fn get_voxel(
        &self,
        position: UVec3,
    ) -> Option<(usize, crate::voxel::cpu::topology::graph::Idx)> {
        self.layers[0].get_voxel_cached(position)
        //self.layers[0].get_voxel(position)
    }
}
