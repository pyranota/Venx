use std::sync::Arc;

use glam::UVec3;

use crate::voxel::segment::Segment;

pub trait LayerInterface {
    // fn new_image(&mut self, name: &str) -> usize;
    /// Set segment with overwriting everything within its bounding box.
    /// Alternatively you can set it with `set_voxel` and than call `compress` on its location
    /// Be aware, that you should do it only if you understand what are u doing
    fn set_segment(&mut self, layer: usize, segment: Segment, position: UVec3);
    fn set_voxel(&mut self, layer: usize, position: UVec3, ty: usize);
    fn compress(&mut self, layer: usize);
    // fn get_image(&self, handle: usize) -> &Image;
    // fn get_image_mut(&mut self, handle: usize) -> &mut Image;

    // fn new_canvas(&mut self, name: &str) -> usize;
    // fn get_canvas(&self, handle: usize) -> &Canvas;
    // fn get_canvas_mut(&mut self, handle: usize) -> &mut Canvas;

    // fn get_layers
    // fn get_layers_mut
    // remove_layer
}
