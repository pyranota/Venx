use easy_compute::ComputeServer;

use crate::{
    shared::byte_casting::any_as_u8_slice,
    voxel::{
        cpu::{self, voxel::Voxel},
        data::VXdata,
        gpu::voxel::VoxelGpu,
        interfaces::voxel::VoxelInterface,
    },
};
/// Abstraction level.
/// Handles all operations of manipulating voxels
/// Its independant from executor (cpu/gpu)
/// Might be removed
pub struct Controller {
    data: VXdata,
    cs: ComputeServer,
}
impl Controller {
    pub(crate) fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Self {
            data: VXdata::Cpu(Voxel::new(depth, chunk_level, segment_level)),
            cs: pollster::block_on(ComputeServer::new()),
        }
    }
    pub fn get_voxel(&self) -> Box<&dyn VoxelInterface> {
        Box::new(match &self.data {
            VXdata::Cpu(vx) => vx,
            VXdata::Gpu(vx) => vx,
        })
    }
    pub fn get_voxel_mut(&mut self) -> Box<&mut dyn VoxelInterface> {
        Box::new(match &mut self.data {
            VXdata::Cpu(vx) => vx,
            VXdata::Gpu(vx) => vx,
        })
    }
}

#[cfg(feature = "gpu")]
impl Controller {
    pub(crate) async fn transfer_to_gpu(self) -> Self {
        let mut gpu_data = VoxelGpu {
            attribute: todo!(),
            topology: todo!(),
        };
        todo!();
        // if let (VXdata::Cpu(cpu_data), cs) = (self.data, self.cs) {
        //     let (attribute, topology) = (cpu_data.attribute, cpu_data.topology);

        //     let attr_buffer = cs.new_buffer(unsafe { any_as_u8_slice(&attribute) });
        // } else {
        //     return self;
        // }
        todo!()
    }
    pub(crate) async fn transfer_to_cpu(self) -> Self {
        if let VXdata::Gpu(gpu_data) = self.data {}
        todo!()
    }
    pub(crate) async fn toggle(self) -> Self {
        todo!()
    }
}
