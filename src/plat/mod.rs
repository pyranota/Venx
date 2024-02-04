use self::{cpu::cpu_plat::CpuPlat, gpu::gpu_plat::GpuPlat};

mod cpu;
mod gpu;
mod interfaces;
#[cfg(feature = "mca_converter")]
mod mca_converter;
mod minecraft_blocks;

// #[derive(Clone)]
pub struct VenxPlat {
    plat: Plat,
}

// #[derive(Debug)]
pub(crate) enum Plat {
    Cpu(CpuPlat),
    #[cfg(feature = "gpu")]
    Gpu(GpuPlat),
}
