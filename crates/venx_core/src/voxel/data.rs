use super::{cpu::voxel::Voxel, gpu::voxel::VoxelGpu};

pub(crate) enum VXdata {
    Cpu(Voxel),
    #[cfg(feature = "gpu")]
    Gpu(VoxelGpu),
}

impl VXdata {
    fn on_cpu(&self) -> bool {
        if matches!(self, VXdata::Cpu(_)) {
            return true;
        }
        false
    }
    #[cfg(feature = "gpu")]
    fn on_gpu(&self) -> bool {
        !self.on_cpu()
    }
}
