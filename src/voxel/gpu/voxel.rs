use std::collections::HashMap;

use easy_compute::{Buffer, ComputePipeline};

use crate::{
    chunk::chunk::Chunk,
    voxel::{
        cpu::{mesh::Mesh, voxel::Voxel},
        interfaces::{layer::LayerInterface, load::LoadInterface, voxel::VoxelInterface},
    },
};

use self::super::state::{CpuOnlyState, OnewaySyncedState, SyncedState};

use super::{attribute::storage::GpuTeTreeStorage, topology::storage::GpuGraphStorage};

#[derive(Debug)]
pub struct VoxelGpu {
    pub cs: ComputeServer,
    bg: ...
    buffers: ...
}

impl LayerInterface for VoxelGpu {
    fn set_segment(
        &mut self,
        layer: usize,
        segment: crate::voxel::segment::Segment,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        todo!()
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn get_voxel(
        &self,
        position: glam::UVec3,
    ) -> Option<(usize, crate::voxel::cpu::topology::graph::Idx)> {
        todo!()
    }
}

impl LoadInterface for VoxelGpu {
    fn load_chunk(&self, position: glam::UVec3, level: u8) -> Chunk {
        todo!()
    }

    fn load_chunks(&self) {
        todo!()
    }

    fn overlay_chunk(&self) {
        todo!()
    }

    fn overlay_chunks(&self) {
        todo!()
    }

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh {
        todo!()
    }
    // fn load_chunk(&self, position: bevy::prelude::UVec3, level: u8) -> Option<Chunk> {
    //     todo!()
    // }

    // fn load_chunks() {
    //     todo!()
    // }

    // fn overlay_chunk() {
    //     todo!()
    // }

    // fn overlay_chunks() {
    //     todo!()
    // }
}

impl VoxelInterface for VoxelGpu {
    // fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
    //     todo!()
    // }
}
