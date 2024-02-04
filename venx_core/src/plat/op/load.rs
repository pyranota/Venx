use spirv_std::glam::UVec3;

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::l2s,
};

use super::LayerOpts;

impl RawPlat {
    fn load_chunk(&self, position: UVec3, level: u8) -> Chunk {
        // TODO change
        let chunk_level = 5;
        let mut chunk = Chunk::new(position, level, chunk_level);
        let chunk_lod_scaler = l2s(level);

        let real_chunk_size = l2s(chunk.level());

        self.traverse_region(
            position,
            chunk_level,
            super::EntryOpts::All,
            LayerOpts::All,
            |node, idx, pos| {
                if node.level == level {
                    chunk.set(*props.position / chunk_lod_scaler, entry as i32);
                    return false;
                }
                true
            },
        );
        // let entries = self.layers[0].graph.entries();
        // // iterate over all entries in graph
        // for entry in 1..entries {
        //     if let Some(chunk_idx) =
        //         self.layers[0]
        //             .graph
        //             .get_node(chunk_level, position * real_chunk_size, entry)
        //     {
        //         self.layers[0].graph.traverse_from(
        //             chunk_idx,
        //             uvec3(0, 0, 0),
        //             chunk_level,
        //             |props| {
        //                 if props.level == level {
        //                     chunk.set(*props.position / chunk_lod_scaler, entry as i32);
        //                     return false;
        //                 }

        //                 true
        //             },
        //         )
        //     }
        // }

        chunk
    }
}
