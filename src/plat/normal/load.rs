use venx_core::plat::chunk::chunk::Chunk;

use crate::plat::interfaces::load::LoadInterface;

use super::cpu_plat::CpuPlat;

impl LoadInterface for CpuPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: u8) -> Chunk {
        self.raw_plat
            .load_chunk(position.to_array().into(), lod_level)
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

    fn compute_mesh_from_chunk<'a>(
        &self,
        chunk: &Chunk,
    ) -> [(
        venx_core::glam::Vec3,
        venx_core::glam::Vec4,
        venx_core::glam::Vec3,
    ); 1000000] {
        self.to_mesh_greedy(chunk)
    }
}
