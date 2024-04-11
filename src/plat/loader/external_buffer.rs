use log::warn;

/// Defines how many bytes can transfered within single call
///
/// Or bus size
pub const EXTERNAL_BUFFER_RESOLUTION: u64 = 1024 * 4;

pub trait ExternalBuffer {
    /// Set Buffer's data
    ///
    /// Should be able to handle ranges atleast [RESOLUTION]
    fn set(self: &Self, offset: u32, data: &[u8]);
}

pub struct FakeBuffer;

impl ExternalBuffer for FakeBuffer {
    fn set(self: &Self, _offset: u32, _data: &[u8]) {
        warn!("Using FakeBuffer")
    }
}
