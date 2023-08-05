use std::collections::HashMap;

use glam::UVec3;

/**
 * Instant is a module with on-fly access and editing capabilities.
 * It stores chunks in 3d greed effectivly (from memory and performance perspective).
 * It allows user to manipulate chunks very easily without sacrifycing anything.
 *
 * It does some part of work inside, but it cant be doing everyting.
 * This is the part, where you should write some code for your integration.
 * It manages automatically and emits events. which chunks should be updated at some point.
 * Programmer should handle this events and respond with some actions.
 * For example:
 *   Instant says:
 *       Hey you moved 100 blocks forward, i think, you should reload this [(0,0,0),(10,1,23)...] chunks at lower level of details
 *   
 *
 */

pub struct Instant {
    pub chunks: HashMap<UVec3, Chunk>,
    /// On what level of topology graph live chunks
    pub chunk_level: u8,
}

pub struct Chunk {
    /// 0 is default
    /// 1: makes this chunk with depth 0 on distance where it should be 1
    /// 2: depth 0 on distance where should be 2
    /// ...
    lvl_offset: u8,
}
