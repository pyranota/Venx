use std::collections::HashMap;

use async_trait::async_trait;
use glam::UVec3;
use venx_core::plat::{node::Node, node_l2::NodeL2, op::get::GetNodeResult};

#[async_trait]
pub trait LayerInterface {
    async fn compress_dag(&mut self, _layer: usize, _position: UVec3, _level: u32) {
        todo!()
    }

    /// Show amount of free nodes on upper level (2..depth) and level 2
    fn free(&self, _layer: usize) -> (u32, u32) {
        todo!()
    }

    /// Freeze layer. Meaning it cannot be mutated anymore.
    ///
    /// It removes all free nodes and make layer as small as possible
    ///
    /// Use [reserve] to unfreeze
    fn freeze(&self, _layer: usize) {
        todo!()
    }

    /// Make sure (upper, l2) have specified amount of nodes
    fn reserve(&self, _layer: usize, _amounts: (u32, u32)) {
        todo!()
    }

    /// Return amount of nodes on upper level (2..depth) and level 2
    fn length(&self, _layer: usize) -> (u32, u32) {
        todo!()
    }

    async fn set_voxel(&mut self, _layer: usize, _position: UVec3, _voxel_id: usize) {
        todo!()
    }

    async fn set_voxel_many(&mut self, _layer: usize, _voxels: (UVec3, usize)) {
        todo!()
    }

    /// Fastest way to set many voxels
    async fn set_segment(&mut self, _segment: ()) {
        todo!();
    }

    // TODO: Make async
    fn get_voxel(&self, _position: UVec3) -> Option<GetNodeResult> {
        todo!()
    }

    #[deprecated = "Use [compress_dag]"]
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
}
