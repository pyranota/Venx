use std::{future::Future, process::Output};

use anyhow::bail;
use glam::{Quat, UVec3, Vec3, Vec4};
use venx_core::{mesh::Mesh, plat::chunk::chunk::Chunk};

use crate::plat::{interfaces::load::LoadInterface, VenxPlat};

use super::BucketIdx;

pub struct ChunkMap {
    pub(super) chunk_mesh_vec: Vec<ChunkMeshInfo>,
}

/// Dont compare ChunkMeshInfo instances with each other. It will check only distance and ignore detailed
pub struct ChunkMeshInfo {
    /// Distance from focus point
    distance: f32,
    detailed: Box<ChunkMeshInfoDetailed>,
}

impl Ord for ChunkMeshInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}
impl Eq for ChunkMeshInfo {}
impl PartialOrd for ChunkMeshInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.distance.partial_cmp(&other.distance) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.distance.partial_cmp(&other.distance)
    }
}
impl PartialEq for ChunkMeshInfo {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

struct ChunkMeshInfoDetailed {
    position: UVec3,
    lod: u8,
    lod_offset: i32,
    /// Tracks down, what buckets are occupied by this chunk.
    ///
    /// Cannot be known instantly
    vertex_pool: Option<VertexPoolChunk>,
}

// TODO: Find more informative name
pub struct VertexPoolChunk {
    pub(crate) occupied_buckets: Vec<usize>,
    pub(crate) face_count: u32,
}

impl ChunkMap {
    pub fn set_focus(&mut self, focus_translation: [f32; 3], focus_rotation: [f32; 4]) {
        let focus_translation = Vec3::from_array(focus_translation);
        let _focus_rotation = Quat::from_array(focus_rotation);

        self.update_distances(focus_translation);

        self.chunk_mesh_vec.sort();

        todo!()
    }

    fn update_chunks(&mut self) {}

    fn update_distances(&mut self, focus_translation: Vec3) {
        for chunk_mesh_info in &mut self.chunk_mesh_vec {
            let distance = chunk_mesh_info
                .detailed
                .position
                .as_vec3()
                .distance(focus_translation);

            chunk_mesh_info.distance = distance;
        }
    }

    /// Get chunk which is farest and will be unloaded first
    // TODO: Prefere culled and largest (in terms of mesh) chunks over visible ones
    fn query_sinking_chunk(&self) {
        todo!()
    }

    pub fn land_chunk(&mut self, chunk: usize, face_count: u32) {
        todo!()
    }

    fn allocate_buckets(&mut self, face_count: u32) -> Vec<usize> {
        todo!()
    }
}
