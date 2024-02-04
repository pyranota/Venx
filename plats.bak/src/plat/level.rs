use bitcode::{Decode, Encode};
use serde::Serialize;

use super::node::Node;

#[derive(Clone, Debug, Encode, Decode)]
pub struct Level {
    /// Link to first node which is empty (flag == -1)
    /// If there is no empty nodes its 0
    pub holder_head: usize,
    /// Every node on level(depth) is entry node
    /// Each entry represents root of graph
    /// That means, that in single `Graph` struc, you can have multiple graphs
    /// That is used to have voxel types in graph
    /// All graphs are merged
    /// By creating new entry you create new graph

    /// Keep in mind that anything on 0 is reserved and not usable
    /// You can identify this types of nodes with 9 in every field of it
    /// This is in that way because if there would be node at 0 index,
    /// that would conflict with 0 as "no child" interpretation
    pub nodes: Vec<Node>,
}
