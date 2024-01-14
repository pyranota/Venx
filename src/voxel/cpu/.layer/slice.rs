use crate::voxel::cpu::topology::graph::Graph;

#[derive(Debug)]
pub struct Slice {
    pub graph: Graph,
    ty: usize,
    merged: bool,
}

impl Slice {
    pub fn new(ty: usize, depth: u8) -> Self {
        Slice {
            graph: Graph::new(depth),
            ty,
            merged: false,
        }
    }
}
