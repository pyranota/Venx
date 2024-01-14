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