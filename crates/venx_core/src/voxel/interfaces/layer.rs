use std::sync::Arc;

use crate::voxel::cpu::layer::{canvas::Canvas, image::Image};

pub trait LayerInterface {
    fn new_image(&mut self, name: &str) -> usize;
    fn get_image(&self, handle: usize) -> &Image;
    fn get_image_mut(&mut self, handle: usize) -> &mut Image;

    fn new_canvas(&mut self, name: &str) -> usize;
    fn get_canvas(&self, handle: usize) -> &Canvas;
    fn get_canvas_mut(&mut self, handle: usize) -> &mut Canvas;

    // fn get_layers
    // fn get_layers_mut
    // remove_layer
}
