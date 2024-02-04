use venx_core::utils::Grid;

use crate::plat::interfaces::load::LoadInterface;

use super::cpu_plat::CpuPlat;

impl LoadInterface for CpuPlat {
    fn load_chunk<const SIZE: usize>(&self, position: glam::UVec3, lod_level: u8) -> Grid<SIZE> {
        todo!()
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

    fn compute_mesh_from_chunk<'a, const SIZE: usize>(
        &self,
        chunk: &Grid<SIZE>,
    ) -> &'a [(glam::Vec3, glam::Vec4, glam::Vec3)] {
        todo!()
    }
}
