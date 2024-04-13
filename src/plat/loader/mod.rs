#![allow(dead_code)]
use glam::{uvec3, Quat, UVec3, Vec3, Vec4};

use self::{
    chunk_map::ChunkMap,
    task_queue::TaskQueue,
    vertex_pool::{ExternalBufferObject, VertexPool},
};

use super::{interfaces::load::LoadInterface, VenxPlat};

pub mod chunk_map;
pub mod external_buffer;
mod task_queue;
pub mod vertex_pool;

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
    pub fn focus_set(&mut self, focus_translation: [f32; 3], focus_rotation: [f32; 4]) {
        self.loader
            .chunk_map
            .set_focus(focus_translation, focus_rotation);
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

impl VenxPlat {
    pub fn land_chunks(&mut self) -> anyhow::Result<()> {
        for x in 0..16 {
            for y in 3..6 {
                for z in 0..16 {
                    dbg!(x, y, z);
                    // Load chunks for the first time
                    let chunk = self.load_chunk(uvec3(x, y, z), 0, 5);

                    let mesh = self.compute_mesh_from_chunk(&chunk);

                    let mut new_mesh = vec![];
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

                        new_mesh.push(pos.to_array());

                        count += 1;
                    }
                    assert_eq!(count % 6, 0);
                    let bucket_amount = count / self.loader.get_bucket_size() + 1;

                    // dbg!(bucket_amount);

                    // dbg!(self.loader.get_bucket_size());

                    // dbg!(count);

                    let bucket_ids = self.loader.vertex_pool.allocate(bucket_amount)?;

                    self.loader.vertex_pool.load_mesh(new_mesh, bucket_ids);
                }
            }
        }
        Ok(())
    }
}
