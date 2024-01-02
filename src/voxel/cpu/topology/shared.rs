use std::collections::HashMap;

use super::{graph::Branch, level::GLevel};

pub struct Shared {
    pub levels: Vec<GLevel>,
    pub level_caches: Vec<LevelCache>,
}

pub struct LevelCache {
    pub map: HashMap<Branch, u32>,
}
