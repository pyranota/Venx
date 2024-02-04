use crate::plat::interfaces::layer::LayerInterface;

use super::gpu_plat::GpuPlat;

impl LayerInterface for GpuPlat {
    fn set_segment<const SIZE: usize>(
        &mut self,
        layer: usize,
        segment: crate::plat::interfaces::Grid<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        todo!()
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<(usize, usize)> {
        todo!()
    }
}
