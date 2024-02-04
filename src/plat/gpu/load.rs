use crate::plat::interfaces::load::LoadInterface;

use super::gpu_plat::GpuPlat;

impl LoadInterface for GpuPlat {
    fn load_chunk<const SIZE: usize>(
        &self,
        position: glam::UVec3,
        lod_level: u8,
    ) -> crate::plat::interfaces::Grid<SIZE> {
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
        chunk: &crate::plat::interfaces::Grid<SIZE>,
    ) -> &'a [(glam::Vec3, glam::Vec4, glam::Vec3)] {
        todo!()
    }
}
