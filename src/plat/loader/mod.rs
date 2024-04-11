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
    pub fn new(initial_focus: (Vec3, Quat, Fov), vertex_pool: VertexPool) -> Self {
        VenxLoader {
            focus: initial_focus,
            chunk_map: ChunkMap {
                chunk_mesh_vec: vec![],
            },
            task_queue: TaskQueue::default(),
            vertex_pool,
        }
    }

    pub fn get_bucket_size(&self) -> u32 {
        self.vertex_pool.bucket_size
    }
    pub fn get_bucket_amount(&self) -> u32 {
        self.vertex_pool.bucket_amount
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
        let mesh = self.compute_mesh_from_chunk(&chunk);

        let mut count = 0;

        'mesh: for attr in mesh.iter() {
            let (pos, color, normal) = (
                Vec3::from_slice(&attr[0..3]),
                Vec4::from_slice(&attr[3..7]),
                Vec3::from_slice(&attr[7..10]),
            );

            // Each returned mesh is static length, so not all attributes in that mesh are used
            // To prevent leaking zero attributes into actual mesh, we check it
            // Dont create blocks with color Vec4::ZERO, it will break the mesh
            if color.to_array() == glam::f32::Vec4::ZERO.to_array() {
                // if count != 0 {
                // dbg!(count / 6);
                // }

                break 'mesh;
            }
            count += 1;
        }

        let bucket_amount = count / self.loader.get_bucket_size() + 1;

        dbg!(bucket_amount);

        dbg!(self.loader.get_bucket_size());

        dbg!(count);

        let bucket_ids = self.loader.vertex_pool.allocate(bucket_amount)?;

        self.loader.vertex_pool.load_mesh(mesh, bucket_ids);
        Ok(())
    }
}
