use easy_compute::ComputeServer;

use crate::voxel::{
    cpu::{self, voxel::Voxel},
    data::VXdata,
    gpu::voxel::VoxelGpu,
    interfaces::voxel::VoxelInterface,
};
/// Abstraction level.
/// Handles all operations of manipulating voxels
/// Its independant from executor (cpu/gpu)
/// Might be removed
// #[derive(Debug)]
