use glam::uvec3;

use crate::{
    chunk::chunk::Chunk,
    voxel::{
        cpu::{utils::lvl_to_size, voxel::Voxel},
        interfaces::load::LoadInterface,
    },
};

/// File corresponding for loading chunks
impl LoadInterface for Voxel {
    /// General way to load chunk, will encounter all layers and slices
    fn load_chunk(&self, position: glam::UVec3) -> crate::chunk::chunk::Chunk {
        let chunk_level = self.chunk_level;
        let mut chunk = Chunk::new(position, 0, self.chunk_level);

        let chunk_size = lvl_to_size::lvl_to_size(chunk_level);

        // iterate over all entries in graph
        for entry in 1..self.layers[0].graph.entries() {
            if let Some(chunk_idx) =
                self.layers[0]
                    .graph
                    .get_node(chunk_level, position * chunk_size, entry)
            {
                self.layers[0].graph.traverse_from(
                    chunk_idx,
                    uvec3(0, 0, 0),
                    chunk_level,
                    |branch, idx, pos, lvl| {
                        //  dbg!(lvl);
                        if lvl == 0 {
                            chunk.set(pos, entry as i32);
                        }
                        true
                    },
                )
            }
        }

        chunk
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

    fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> crate::voxel::cpu::mesh::Mesh {
        self.to_mesh_naive(chunk)
    }
}
