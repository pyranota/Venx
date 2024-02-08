use std::borrow::BorrowMut;

use venx_core::utils::Grid;

use crate::plat::interfaces::layer::LayerInterface;

use super::cpu_plat::CpuPlat;

impl LayerInterface for CpuPlat {
    fn set_segment<const SIZE: usize>(
        &mut self,
        layer: usize,
        segment: Grid<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        self.with_raw_plat_mut(|plat| plat[layer].set(position.to_array().into(), ty as u32));
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<usize> {
        self.borrow_raw_plat().get_voxel(position.to_array().into())
    }
}
