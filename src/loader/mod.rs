use std::collections::HashMap;

use glam::{UVec3, Vec3};

/// Helps to load/unload chunks
pub struct VenxLoader {
    /// Where player is
    pub focus_position: Vec3,
    /// Rendering distance
    pub vertices_amount: usize,
    /// Maximum lod level
    /// Above that level there will be no loaded chunks
    pub max_lod_level: usize,
    /// Shows what chunk and what level are loaded
    pub chunks: VoxTree<()>,
    /// Says how much each lod level can take vertices
    /// Input is current amount of vertices, output is which level should write
    vertices_formule: dyn Fn(usize) -> usize,
}

impl VenxLoader {
    /// Update player position
    /// It will validate all loaded chunks
    /// Returns chunks that should be reloaded
    pub fn update(&mut self, focus_position: Vec3) {
        self.focus_position = focus_position;
        for (position, level) in &self.chunks {
            todo!()
        }
    }
}
