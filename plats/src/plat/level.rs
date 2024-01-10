use bitcode::{Decode, Encode};
use serde::Serialize;

use super::node::Node;

#[derive(Clone, Debug, Encode, Decode)]
pub struct Level {
    pub holder_head: usize,
    pub nodes: Vec<Node>,
}
