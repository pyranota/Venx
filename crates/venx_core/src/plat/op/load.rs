use spirv_std::glam::UVec3;

use crate::plat::{chunk::chunk::Chunk, layer::layer::Layer};

impl Layer<'_> {
    #[inline(always)]
    pub fn load_chunk_gpu(&self, chunk: &mut Chunk) {
        self.traverse(chunk.position(), 0..=(chunk.chunk_level()), |p| {
            if p.level == 0 {
                chunk.set_global(*p.position, p.voxel_id as u32);
            }
        });
    }

    pub fn load_chunk(&self, position: UVec3, lod_level: usize, chunk_level: usize) -> Chunk {
        let mut chunk = Chunk::new(position, lod_level, chunk_level);
        self.load_chunk_gpu(&mut chunk);
        chunk
    }
}
#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use std::dbg;

    use spirv_std::glam::{uvec3, UVec3};

    use crate::{plat::layer::layer::Layer, quick_raw_plat};

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

        plat[0].traverse(UVec3::ONE, 0..=5, |p| {
            if p.level == 0 {
                dbg!(p.position);
            }
        });

        let chunk = plat[Layer::BASE].load_chunk(uvec3(1, 1, 1), 0, 5);
        assert!(chunk.get_global(uvec3(61, 61, 61)).is_none());
        assert!(chunk.get_global(uvec3(60, 60, 60)).is_some());
    }
}
