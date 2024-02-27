use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{
        chunk::chunk::{Chunk, ChunkMeta},
        layer::layer::Layer,
        node::NodeAddr,
        raw_plat::RawPlat,
    },
    utils::l2s,
};

impl Layer<'_> {
    #[inline(always)]
    pub fn load_chunk_gpu(&self, chunk: &mut Chunk) {
        // for x in 0..chunk.size() {
        //     for y in 0..chunk.size() {
        //         for z in 0..chunk.size() {
        //             let res =
        //                 self.get_node(uvec3(x, y, z) + chunk.position() * chunk.width(), 0, None);

        //             if res.is_some() {
        //                 chunk.set(uvec3(x, y, z), res.voxel_id as u32);
        //             }
        //         }
        //     }
        // }
        // #[cfg(feature = "bitcode_support")]
        // panic!("{:?}", chunk.position());

        self.traverse_new(chunk.position(), 0..=(chunk.chunk_level()), |p| {
            if p.level == 0 {
                if p.entry != 0 {
                    chunk.set_global(*p.position, p.entry as u32);
                }
            }
        });

        // let node_idx = self.get_node_idx_gpu(chunk.position() * chunk.width(), chunk.chunk_level());

        // if node_idx != 0 {
        //     self.traverse_gpu(0, node_idx, UVec3::ZERO, true, 5, |(level, entry, p)| {
        // if level == 0 {
        //     if entry != 0 {
        //         chunk.set(p, entry as u32);
        //     }
        // }
        //     });
        // }
    }

    pub fn load_chunk(&self, position: UVec3, lod_level: usize, chunk_level: usize) -> Chunk {
        let mut chunk = Chunk::new(position, lod_level, chunk_level);

        self.load_chunk_gpu(&mut chunk);

        chunk
    }
}
#[cfg(feature = "bitcode_support")]
#[cfg(test)]
mod tests {
    use std::{dbg, println};

    use alloc::vec;
    use spirv_std::glam::{uvec3, UVec3};

    use crate::{
        plat::{chunk::chunk::Chunk, layer::layer::Layer, node::Node, raw_plat::RawPlat},
        quick_raw_plat, *,
    };

    extern crate alloc;
    extern crate std;
    #[test]
    fn load_chunk() {
        quick_raw_plat!(plat, depth 12, len 1_000);

        plat[0].set(uvec3(15, 15, 15), 1);
        plat[0].set(uvec3(0, 0, 0), 2);
        plat[0].set(uvec3(60, 60, 60), 12);

        let chunk = plat[Layer::BASE].load_chunk(uvec3(0, 0, 0), 0, 5);

        assert!(chunk.get(uvec3(0, 0, 0)).is_some());
        assert!(chunk.get(uvec3(15, 15, 15)).is_some());
        assert!(chunk.get(uvec3(5, 15, 5)).is_none());
        // Out of bound
        assert!(chunk.get(uvec3(15, 150, 15)).is_none());
        assert!(chunk.get(uvec3(60, 60, 60)).is_none());

        // plat[0].traverse_new(UVec3::ZERO, 0..=6, |p| {
        //     if p.level == 0 {
        //         dbg!(p.position);
        //     }
        // });

        plat[0].traverse_new(UVec3::ONE, 0..=5, |p| {
            if p.level == 0 {
                dbg!(p.position);

                //let mut chunk = Chunk::new(UVec3::ONE, 0, 5);

                //chunk.set_global(*p.position, 2);
            }
        });

        let chunk = plat[Layer::BASE].load_chunk(uvec3(1, 1, 1), 0, 5);
        assert!(chunk.get_global(uvec3(61, 61, 61)).is_none());
        assert!(chunk.get_global(uvec3(60, 60, 60)).is_some());
    }
}
