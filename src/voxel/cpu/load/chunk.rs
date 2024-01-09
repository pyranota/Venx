use glam::uvec3;

use crate::{
    chunk::chunk::Chunk,
    voxel::{
        cpu::{
            traverse::TrProps,
            utils::lvl_to_size::{self, lvl_to_size},
            voxel::Voxel,
        },
        interfaces::load::LoadInterface,
    },
};

/// File corresponding for loading chunks
impl LoadInterface for Voxel {
    /// General way to load chunk, will encounter all layers and slices
    fn load_chunk(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
        let chunk_level = self.chunk_level;
        let mut chunk = Chunk::new(position, level, self.chunk_level);
        let chunk_lod_scaler = lvl_to_size(level);

        let real_chunk_size = lvl_to_size(chunk.level());
        let entries = self.layers[0].graph.entries();
        // iterate over all entries in graph
        for entry in 1..entries {
            if let Some(chunk_idx) =
                self.layers[0]
                    .graph
                    .get_node(chunk_level, position * real_chunk_size, entry)
            {
                self.layers[0].graph.traverse_from(
                    chunk_idx,
                    uvec3(0, 0, 0),
                    chunk_level,
                    |props| {
                        // if let TrProps::Leaf { position, .. } = props {
                        //     chunk.set(*position, entry as i32);
                        // }

                        if props.level == level {
                            chunk.set(*props.position / chunk_lod_scaler, entry as i32);
                            return false;
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
