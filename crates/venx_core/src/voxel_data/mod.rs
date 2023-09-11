use self::{cpu_data::CpuData, gpu_data::GpuData};
mod cpu_data;

#[cfg(feature = "gpu")]
mod gpu_data;

pub(crate) enum VXdata {
    Cpu(CpuData),
    #[cfg(feature = "gpu")]
    Gpu(GpuData),
}
