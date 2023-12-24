use std::collections::HashMap;

use venx_compute::{Buffer, ComputePipeline};

use crate::{
    chunk::chunk::Chunk,
    voxel::{
        cpu::{mesh::Mesh, voxel::Voxel},
        interfaces::{layer::LayerInterface, voxel::VoxelInterface},
    },
};

use self::super::state::{CpuOnlyState, OnewaySyncedState, SyncedState};

use super::{attribute::storage::GpuTeTreeStorage, topology::storage::GpuGraphStorage};

#[derive(Debug)]
pub struct VoxelGpu {
    pub attribute: GpuTeTreeStorage,
    pub topology: GpuGraphStorage,
}

impl LayerInterface for VoxelGpu {
    fn new_image(&mut self, name: &str) -> usize {
        todo!()
    }

    fn new_canvas(&mut self, name: &str) -> usize {
        todo!()
    }

    fn get_image(&self, handle: usize) -> &crate::voxel::cpu::layer::image::Image {
        todo!()
    }

    fn get_image_mut(&mut self, handle: usize) -> &mut crate::voxel::cpu::layer::image::Image {
        todo!()
    }

    fn get_canvas(&self, handle: usize) -> &crate::voxel::cpu::layer::canvas::Canvas {
        todo!()
    }

    fn get_canvas_mut(&mut self, handle: usize) -> &mut crate::voxel::cpu::layer::canvas::Canvas {
        todo!()
    }
}

impl VoxelInterface for VoxelGpu {
    // fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
    //     todo!()
    // }

    fn load_chunk(
        &self,
        position: glam::UVec3,
        level: u8,
    ) -> std::option::Option<crate::chunk::chunk::Chunk> {
        todo!()
    }

    fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        todo!()
    }

    fn load_chunk_n_mesh(&self) {
        todo!()
    }

    fn load_chunks_n_meshes(&self) {
        todo!()
    }

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh {
        todo!()
    }

    fn get(&self, level: u8, position: glam::UVec3) -> Option<usize> {
        todo!()
    }
}
