#[cfg(feature = "mca_converter")]
mod mca_converter;

use bevy::prelude::Component;

use crate::{
    chunk::{chunk::Chunk, storage::ChunksStorage},
    controller::Controller,
    voxel::{
        cpu::mesh::Mesh,
        interfaces::{layer::LayerInterface, voxel::VoxelInterface},
        segment::Segment,
    },
};

pub struct Plat {
    pub controller: Controller,
    chunks: ChunksStorage,
}

impl std::fmt::Debug for Plat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plat")
            // .field("controller", &self.controller)
            // .field("chunks", &self.chunks)
            .finish()
    }
}

#[cfg(feature = "bevy_ecs")]
impl Component for Plat {
    type Storage;
}

impl Plat {
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Plat {
            controller: Controller::new(depth, chunk_level, segment_level),
            chunks: ChunksStorage {},
        }
    }
    pub fn load() {
        todo!()
    }
    pub fn save() {
        todo!()
    }
    pub fn new_segment(&self) -> Segment {
        // Segment::new(self.se)
        todo!()
    }
}

impl LayerInterface for Plat {
    fn new_canvas(&mut self, name: &str) -> usize {
        self.controller.get_voxel_mut().new_canvas(name)
    }

    fn new_image(&mut self, name: &str) -> usize {
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
        self.controller.get_voxel_mut().get_canvas_mut(handle)
    }
}
// fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
//     self.controller
//         .get_voxel_mut()
//         .insert_segment(segment, position);
// }

impl VoxelInterface for Plat {
    fn load_chunk(
        &self,
        position: glam::UVec3,
        level: u8,
    ) -> std::option::Option<crate::chunk::chunk::Chunk> {
        self.controller.get_voxel().load_chunk(position, level)
    }

    fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        self.controller.get_voxel().load_chunks(position, level)
    }

    fn load_chunk_n_mesh(&self) {
        self.controller.get_voxel().load_chunk_n_mesh()
    }

    fn load_chunks_n_meshes(&self) {
        self.controller.get_voxel().load_chunks_n_meshes()
    }

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh {
        self.controller.get_voxel().compute_mesh_from_chunk(chunk)
    }

    fn get(&self, level: u8, position: glam::UVec3) -> Option<usize> {
        self.controller.get_voxel().get(level, position)
    }
}
