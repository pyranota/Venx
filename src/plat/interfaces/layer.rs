use std::collections::HashMap;

use async_trait::async_trait;
use glam::UVec3;
use venx_core::plat::{node::Node, node_l2::NodeL2, op::get::GetNodeResult};

#[async_trait]
pub trait LayerInterface {
    async fn compress_dag(&mut self, _layer: usize, _position: UVec3, _level: u32) {
        todo!()
    }

    async fn set_voxel(&mut self, _layer: usize, _position: UVec3, _voxel_id: usize) {
        todo!()
    }

    async fn set_voxel_many(&mut self, _layer: usize, _voxels: (UVec3, usize)) {
        todo!()
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
