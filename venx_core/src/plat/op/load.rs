use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::l2s,
};

use super::LayerOpts;

impl RawPlat {
    pub fn load_chunk(&self, position: UVec3, lod_level: u8) -> Chunk {
        // TODO change
        let chunk_level = 5;
        let mut chunk = Chunk::new(position, lod_level, chunk_level);
        let chunk_lod_scaler = l2s(lod_level);

        // let real_chunk_size = l2s(chunk.level());

        self.traverse_region(
            position,
            chunk_level,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |props| {
                if props.level == lod_level {
                    chunk.set(props.position.unwrap() / chunk_lod_scaler, props.entry);
                    return false;
                }
                true
            },
        );
        chunk
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use spirv_std::glam::uvec3;

    use crate::plat::raw_plat::RawPlat;

    extern crate std;
    #[test]
    fn load_chunk() {
        let mut plat = RawPlat::new(6, 5, 5);

        plat[0].set(uvec3(15, 15, 15), 1);
        plat[0].set(uvec3(0, 0, 0), 2);

        let chunk = plat.load_chunk(uvec3(0, 0, 0), 0);

        //println!("{:?}", chunk);

        assert!(chunk.get(uvec3(0, 0, 0)).is_some());
    }
}
