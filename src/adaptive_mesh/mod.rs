use std::collections::HashMap;

use glam::UVec3;

/// It is the way to edit uploaded mesh on gpu very quickly
///
/// We have chunks, each chunk stores handle to vector
/// Each element in vector going along with 6 vertices and contain link to next 6 vertices
/// With this approach we can easily modify vertices. We can add/change/remove chunks with speed of light.
///
/// Ideally game-engine integration should allow venx directly access mesh buffer on gpu and mutate it
/// without additional steps on cpu
pub struct AdaptiveMesh {
    /// Small overview of loaded chunks
    /// HashMap<Chunk position, (SubMesh idx, first Vertex idx of chunk)>
    chunks: HashMap<UVec3, (usize, usize)>,
    /// Why not to use 1?
    /// If you have only one mesh and it grows bigger,
    /// grows also chance that your mesh could not be allocated on gpu
    /// Its much more effective to have 10-20 meshes and render each in seprate draw call
    /// It also makes concurrent modification possible
    sub_meshes: Vec<SubMesh>,
}

pub struct SubMesh {
    /// head of unused vertices
    unused: usize,
    /// Links should by 6 times smaller than vertex buffer
    links: Vec<usize>,
}

impl AdaptiveMesh {
    /// Unloads chunk from mesh overview
    /// And removes all links to it
    /// Removed links are appended to unused links
    /// Thay can be reused after
    pub fn unload(&mut self, chunk_location: UVec3) {
        todo!()
    }
    /// Register chunk
    /// It allocates chunk in chunks field
    /// Returns (Chunk root id, &mut Links)
    /// Links should be properly modified by game engine integration implementation
    pub fn register<'a>(&mut self, chunk_location: UVec3) -> (&usize, &mut Vec<usize>) {
        todo!()
    }
}
