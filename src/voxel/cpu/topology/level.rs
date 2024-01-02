use std::ops::{Index, IndexMut};

use super::graph::Branch;

#[derive(Default, Clone, Debug)]
pub struct GLevel {
    pub nodes: Vec<Branch>,
}

impl Index<usize> for GLevel {
    type Output = Branch;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index - 1] // -1
    }
}

impl IndexMut<usize> for GLevel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index - 1] // -1
    }
}
