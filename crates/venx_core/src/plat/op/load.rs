use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::l2s,
};

use super::LayerOpts;

impl RawPlat<'_> {
    pub fn load_chunk(&self, position: UVec3, lod_level: usize) -> Chunk {
        // TODO change
        let chunk_level = 5;
        let mut chunk = Chunk::new(position, lod_level, chunk_level);

        let chunk_lod_scaler = l2s(lod_level);

        // // let real_chunk_size = l2s(chunk.level());

        self.traverse_region(
            position,
            chunk_level,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |props| {
                let props = props;
                if props.level == lod_level {
                    chunk.set(*props.position / chunk_lod_scaler, props.entry);
                    props.drop_tree = true;
                }
            },
        );

        chunk
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use alloc::vec;
    use spirv_std::glam::uvec3;

    use crate::plat::{node::Node, raw_plat::RawPlat};

    extern crate alloc;
    extern crate std;
    #[test]
    fn load_chunk() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            6,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        plat[0].set(uvec3(15, 15, 15), 1);
        plat[0].set(uvec3(0, 0, 0), 2);

        let chunk = plat.load_chunk(uvec3(0, 0, 0), 0);

        //println!("{:?}", chunk);

        assert!(chunk.get(uvec3(0, 0, 0)).is_some());
    }
}
