#![allow(dead_code)]
use glam::{Quat, Vec3};

use self::{
    chunk_map::ChunkMap,
    task_queue::TaskQueue,
    vertex_pool::{ExternalBufferObject, VertexPool},
};

use super::VenxPlat;

pub mod chunk_map;
pub mod external_buffer;
mod task_queue;
mod vertex_pool;

pub type BucketIdx = usize;
pub type Fov = usize;

// TODO: Opt out with feature / allow user implement its own loader
/// Manages loading meshes and chunks
pub struct VenxLoader {
    /// Position of camera in world
    pub(crate) focus: (Vec3, Quat, Fov),
    pub(crate) chunk_map: ChunkMap,
    pub(crate) task_queue: TaskQueue,
    pub(crate) vertex_pool: VertexPool,
}

impl VenxLoader {
    pub fn new(
        initial_focus: (Vec3, Quat, Fov),
        bucket_size: u32,
        bucket_amount: u32,
        indirect_buffer: ExternalBufferObject,
        vertex_buffer: ExternalBufferObject,
    ) -> Self {
        VenxLoader {
            focus: initial_focus,
            chunk_map: ChunkMap {
                chunk_mesh_vec: vec![],
            },
            task_queue: TaskQueue::default(),
            vertex_pool: VertexPool::new(
                bucket_size,
                bucket_amount,
                indirect_buffer,
                vertex_buffer,
            ),
        }
    }
}

impl VenxPlat {
    /// Rotation is a Quaternion!
    /// You need to run this method each frame in order to update chunks
    pub fn focus_set(&mut self, _focus_translation: [f32; 3], _focus_rotation: [f32; 4]) {
        todo!()
    }

    pub fn focus_set_fov(&mut self, _new_fov: usize) {
        todo!()
    }

    pub fn culling_disable(&mut self) {
        todo!()
    }
    /// Enabled by default
    pub fn culling_enable(&mut self) {
        todo!()
    }

    /// Change vertex pool's properties such as bucket_size and bucket_count.
    /// Forces reloading of all chunks
    pub fn rendering_distance_reset(&mut self) {
        todo!()
    }
}
