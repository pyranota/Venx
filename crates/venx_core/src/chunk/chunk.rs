use glam::UVec3;

pub struct Chunk {
    pub mtx: Vec<Vec<Vec<bool>>>,
    pub position: UVec3,
    pub lod_level: u8,
}
