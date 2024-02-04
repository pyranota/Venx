use venx_core::plat::raw_plat::RawPlat;

use crate::plat::interfaces::PlatInterface;

#[derive(Debug, Clone)]
pub struct GpuPlat {
    raw_plat: RawPlat,
}

impl PlatInterface for GpuPlat {}
