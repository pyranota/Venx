use std::mem::ManuallyDrop;

use glam::UVec3;

use super::level::GLevel;

pub type Idx = usize;

#[derive(Debug)]
pub struct Graph {
    pub(crate) depth: u32,
    pub levels: Vec<GLevel>,
    pub empty_head: Idx,
    pub root: Idx,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Branch {
    /// 0 - normal branch, 1 - link to shared, -1 - empty node
    /// if its `-1`, first child will be interpreted as link to the next empty node
    /// And second child as previous node. If there is no nodes it will be `0`
    pub ident: i32,
    /// If branch is on 1 level, than all children are identified as blocks
    /// u32 == u24 // Each layer can be maximum 500mb
    pub children: [u32; 8],
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
