use crate::voxel::{
    cpu::topology::graph::Graph,
    interfaces::{canvas::CanvasInterface, chunk_loader::ChunkLoaderInterface},
};

#[derive(Debug)]
pub struct Canvas {
    pub graph: Graph,
}

impl CanvasInterface for Canvas {
    fn finalize_into_image() {
        todo!()
    }

    fn insert() {
        todo!()
    }

    fn remove() {
        todo!()
    }

    fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
        log::info!("Inserting segment");
        let offset = segment.size() * position;
        let mut x = 0;
        segment.iter(|pos, block| {
            if pos.x > x {
                x = pos.x;
                log::info!("{x}");
            }
            // Redo
            // log::info!("{}", &pos);

            if block != 0 {
                let _ = self.graph.set(offset + pos, block);
            }
        });
        //self.attribute.optimize();
        log::info!("Segment is inserted");
    }
}

impl ChunkLoaderInterface for Canvas {
    fn load_chunk(
        &self,
        position: bevy::prelude::UVec3,
        level: u8,
    ) -> Option<crate::chunk::chunk::Chunk> {
        todo!()
    }

    fn load_chunks() {
        todo!()
    }

    fn overlay_chunk() {
        todo!()
    }

    fn overlay_chunks() {
        todo!()
    }
}
