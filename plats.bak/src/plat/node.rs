use bitcode::{Decode, Encode};
use serde::Serialize;

#[derive(Clone, Debug, Encode, Decode)]
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
