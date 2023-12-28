use super::graph::Branch;

pub struct SharedLevel {
    level: u8,
    nodes: Branch,
}

pub struct Shared {
    levels: Vec<SharedLevel>,
}
