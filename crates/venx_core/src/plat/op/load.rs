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

use super::LayerOpts;

impl Layer<'_> {
    #[inline(always)]
    pub fn load_chunk_gpu(&self, chunk: &mut Chunk) {
        let node_idx = self.get_node_idx_gpu(chunk.position() * chunk.width(), chunk.chunk_level());

        if node_idx != 0 {
            self.traverse_gpu(0, node_idx, UVec3::ZERO, true, 5, |(level, entry, p)| {
                if level == 0 {
                    if entry != 0 {
                        chunk.set(p, entry as u32);
                    }
                }
            });
        }
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
    use std::println;

    use alloc::vec;
    use spirv_std::glam::uvec3;

    use crate::{
        plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat},
        quick_raw_plat, *,
    };

    extern crate alloc;
    extern crate std;
    #[test]
    fn load_chunk() {
        quick_raw_plat!(plat, depth 6, len 1_000);

        plat[0].set(uvec3(15, 15, 15), 1);
        plat[0].set(uvec3(0, 0, 0), 2);

        let chunk = plat[Layer::BASE].load_chunk(uvec3(0, 0, 0), 0, 5);

        assert!(chunk.get(uvec3(0, 0, 0)).is_some());
        assert!(chunk.get(uvec3(15, 15, 15)).is_some());
        assert!(chunk.get(uvec3(5, 15, 5)).is_none());
        // Out of bound
        assert!(chunk.get(uvec3(15, 150, 15)).is_none());
    }
}
