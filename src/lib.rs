// mod topology;
#![feature(core_intrinsics)]
pub mod plat;
#[cfg(feature = "turbo")]
pub use easy_compute::*;
pub use venx_core::plat::chunk::chunk::*;
