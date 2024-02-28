use std::{collections::HashMap};

use glam::UVec3;
use venx_core::{
    plat::{node::Node, node_l2::NodeL2, op::get::GetNodeResult},
    utils::Grid,
};

pub trait LayerInterface {
    // fn new_image(&mut self, name: &str) -> usize;
    /// Set segment with overwriting everything within its bounding box.
    /// Alternatively you can set it with `set_voxel` and than call `compress` on its location
    /// Be aware, that you should do it only if you understand what are u doing
    fn set_segment<const SIZE: usize>(
        &mut self,
        _layer: usize,
        _segment: Grid<SIZE>,
        _position: UVec3,
    ) {
        todo!()
    }
    fn set_voxel(&mut self, _layer: usize, _position: UVec3, _ty: usize) {
        todo!()
    }
    fn compress(
        &mut self,
        _layer: usize,
        _position: UVec3,
        _level: u32,
        _lookup_tables: &mut Vec<HashMap<Node, usize>>,
        _lookup_table_l2: &mut HashMap<NodeL2, usize>,
    ) {
        todo!()
    }

    fn get_voxel(&self, _position: UVec3) -> Option<GetNodeResult> {
        todo!()
    }
    // fn get_image(&self, handle: usize) -> &Image;
    // fn get_image_mut(&mut self, handle: usize) -> &mut Image;

    // fn new_canvas(&mut self, name: &str) -> usize;
    // fn get_canvas(&self, handle: usize) -> &Canvas;
    // fn get_canvas_mut(&mut self, handle: usize) -> &mut Canvas;

    // fn get_layers
    // fn get_layers_mut
    // remove_layer
}
