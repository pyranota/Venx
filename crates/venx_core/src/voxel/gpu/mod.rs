use venx_compute::Buffer;

use self::state::{CpuOnlyState, OnewaySyncedState, SyncedState};

mod attribute;
mod state;
mod topology;

pub struct GpuGraphStorage {
    buffer: Buffer,
    synced: SyncedState,
    read_only: OnewaySyncedState,
    local: CpuOnlyState,
}
