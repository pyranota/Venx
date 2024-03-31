#![cfg_attr(target_arch = "spirv", no_std)]

pub mod plat;
pub mod utils;
pub use spirv_std::glam;
pub mod mesh;

pub use plat::chunk::chunk::ChunkLoadRequest;
pub use plat::raw_plat::LayerIndex::*;
pub use utils::*;
