use super::node::Node;

pub struct TopoGraph {
    /// power of 2
    depth: u8,
    root: Node,
    nodes: Vec<Node>,
}

impl TopoGraph {
    pub fn new(size: u32) -> Self {
        TopoGraph {
            depth: 128,
            root: Node::default(),
            nodes: vec![],
        }
    }
}
