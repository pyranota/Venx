use std::{collections::HashMap, mem::ManuallyDrop};

use bitcode::{Decode, Encode};
use glam::UVec3;

use super::{level::GLevel, lookup_level::LookupLevel, shared::Shared};

pub type Idx = usize;

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Graph {
    /// Maximal depth of graph, can be extended and/or shrinked
    /// 2^depth represents maximum world size
    #[bitcode_hint(expected_range = "0..20")]
    pub depth: u32,
    /// Nodes are organized in levels. That helps to instantly get all nodes at same level
    /// Each level contains only nodes that are referenced only one time
    /// You can safely edit this graph aslong it does not contain link to shared storage
    pub levels: Vec<GLevel>,
    /// Used for quick merging
    /// Basically its doubling the size in ram
    /// But its not getting serialize
    /// Does not exist for finished layers
    pub lookup_levels: Vec<LookupLevel>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct Branch {
    /// `0` - normal branch,
    /// `1` - link to shared,
    /// `-1` - empty node,
    /// `2` - tmp link to node on same level.
    /// `3` - leaf node (single node at 0 level with idx 1)
    /// `9` - not usable reserved node.
    /// if its `-1`, first child will be interpreted as link to the next empty node
    /// And second child as previous node. If there is no nodes it will be `0`
    pub ident: i32,
    /// If branch is on 1 level, than all children are identified as blocks
    /// u32 == u24 // Each layer can be maximum 500mb
    pub children: [u32; 8],
    // After reading this, you might be wondering,
    // Why not to use enums or any other data type
    // Thats why the same alghorithm should be implemented on cpu and gpu.
    // Howewer on gpu there is no enums and that fancy data structs
    // And for making things uniform and similar it uses that simple types.
    // Plus its makes much easier to convert and send to gpu.
}

impl Branch {
    /// Internal index of node converted to normalized vector
    pub fn get_child_position(i: u32) -> UVec3 {
        UVec3::new(i & 1, (i >> 1) & 1, (i >> 2) & 1)
    }
    /// Convert position of node in 3d space coordinate to internal child branch index
    pub fn get_child_index(pos: UVec3, level: u8) -> usize {
        let child_size = 1 << level;
        let x = if pos[0] < child_size { 0 } else { 1 };
        let y = if pos[1] < child_size { 0 } else { 1 };
        let z = if pos[2] < child_size { 0 } else { 1 };
        (x + y * 2 + z * 4) as usize
    }
    pub fn new(level: u8, ident: i32) -> Self {
        Self {
            ident,
            children: Default::default(),
        }
    }
}
