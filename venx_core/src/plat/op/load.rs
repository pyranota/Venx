use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::l2s,
};

use super::LayerOpts;

impl RawPlat {
    pub fn load_chunk(&self, position: UVec3, level: u8) -> Chunk {
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
            &mut |props| {
                if props.level == level {
                    chunk.set(props.position.unwrap() / chunk_lod_scaler, props.entry);
                    return false;
                }
                true
            },
        );
        chunk
    }
}

#[test]
fn load_chunk() {
    let mut plat = RawPlat::new(7, 5, 6);

    plat[0].set(uvec3(0, 0, 0), 2);

    let chunk = plat.load_chunk(UVec3::ZERO, 0);

    assert!(chunk.get(uvec3(0, 0, 0)).is_some());
}
