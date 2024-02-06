use easy_compute::Buffer;
use venx_core::plat::raw_plat::RawPlat;

use crate::plat::interfaces::PlatInterface;

#[derive(Debug)]
pub struct GpuPlat {
    pub raw_plat: Buffer,
}

impl PlatInterface for GpuPlat {}
