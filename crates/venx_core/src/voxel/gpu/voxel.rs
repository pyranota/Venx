use std::collections::HashMap;

use venx_compute::{Buffer, ComputePipeline};

use self::super::state::{CpuOnlyState, OnewaySyncedState, SyncedState};

use super::{attribute::storage::GpuTeTreeStorage, topology::storage::GpuGraphStorage};

pub struct VoxelGpu {
    pub attribute: GpuTeTreeStorage,
    pub topology: GpuGraphStorage,
}
