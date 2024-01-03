use std::collections::HashMap;

use super::{graph::Branch, level::GLevel};

#[derive(Debug)]
pub struct Shared {
    pub levels: Vec<GLevel>,
    pub level_caches: Vec<LevelCache>,
}
#[derive(Debug, Clone)]
pub struct LevelCache {
    pub map: HashMap<Branch, u32>,
}
