use venx_compute::ComputeServer;

use crate::{
    shared::byte_casting::any_as_u8_slice,
    voxel::{cpu, data::VXdata, gpu::voxel::VoxelGpu},
};

pub(crate) struct Controller {
    data: VXdata,
    cs: ComputeServer,
}

#[cfg(feature = "gpu")]
impl Controller {
    pub(crate) fn new() -> Self {
        todo!()
    }
    pub(crate) async fn transfer_to_gpu(self) -> Self {
        let mut gpu_data = VoxelGpu {
            attribute: todo!(),
            topology: todo!(),
        };

        if let (VXdata::Cpu(cpu_data), cs) = (self.data, self.cs) {
            let (attribute, topology) = (cpu_data.attribute, cpu_data.topology);

            let attr_buffer = cs.new_buffer(unsafe { any_as_u8_slice(&attribute) });
        } else {
            return self;
        }
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
