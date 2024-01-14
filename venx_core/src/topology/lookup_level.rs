use std::collections::HashMap;

use super::graph::{Branch, Idx};

#[derive(Default, Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct LookupLevel {
    pub links: HashMap<[u32; 8], Idx>,
}
