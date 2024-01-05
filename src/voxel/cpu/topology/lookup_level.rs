use std::collections::HashMap;

use super::graph::{Branch, Idx};

#[derive(Default, Clone, Debug)]
pub struct LookupLevel {
    pub links: HashMap<[u32; 8], Idx>,
}
