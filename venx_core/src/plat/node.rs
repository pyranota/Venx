use spirv_std::glam::UVec3;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Node {
    /// `0` - normal branch,
    /// `1` - link to shared,
    /// `-1` - empty node,
    /// `2` - tmp link to node on same level.
    /// `3` - leaf node (single node at 0 level with idx 1)
    /// `9` - not usable reserved node.
    /// if its `-1`, first child will be interpreted as link to the next empty node
    /// And second child as previous node. If there is no nodes it will be `0`
    pub flag: i32,
    // After reading this, you might be wondering,
    // Why not to use enums or any other data type
    // Thats why the same alghorithm should be implemented on cpu and gpu.
    // Howewer on gpu there is no enums and that fancy data structs
    // And for making things uniform and similar it uses that simple types.
    // Plus its makes much easier to convert and send to gpu.
    pub children: [u32; 8],
}

impl Node {
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
    pub fn new(level: u8, flag: i32) -> Self {
        Self {
            flag,
            children: Default::default(),
        }
    }
}
