pub struct Segment {
    level: u8,
    pub mtx: Vec<Vec<Vec<u32>>>,
}
pub struct SegmentStatic<const SIZE: usize> {
    pub mtx: [[[u32; SIZE]; SIZE]; SIZE],
}
