const RESOLUTION: u64 = 100;

pub trait ExternalBuffer {
    /// Should be able to handle ranges atleast [RESOLUTION]
    fn set(self: &Self, bounds: (u64, u64), data: Vec<u8>);
}

pub struct FakeBuffer;

impl ExternalBuffer for FakeBuffer {
    fn set(self: &Self, _bounds: (u64, u64), _data: Vec<u8>) {
        // Yeah :)
    }
}
