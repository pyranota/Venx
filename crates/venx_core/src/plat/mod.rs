use bevy::prelude::Component;

use crate::{chunk::storage::ChunksStorage, controller::Controller, voxel::vx_trait::VoxelTrait};

pub struct Plat {
    controller: Controller,
    chunks: ChunksStorage,
}

#[cfg(feature = "bevy_ecs")]
impl Component for Plat {
    type Storage;
}

impl Plat {
    pub fn load() {}
    pub fn save() {}
    pub fn load_mca() {}
}

impl VoxelTrait for Plat {
    fn insert_segment<const SIZE: usize>(
        &mut self,
        segment: crate::voxel::segment::SegmentStatic<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn load_chunk(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunk_n_mesh() {
        todo!()
    }

    fn load_chunks_n_meshes() {
        todo!()
    }

    fn compute_mesh_from_chunk() {
        todo!()
    }
}
