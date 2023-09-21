use std::ops::{Index, Range};

use super::tetree::TeTree;

impl TeTree {
    // pub(crate) fn remove() {
    //     todo!()
    // }

    // pub(crate) fn optimize() {
    //     todo!()
    // }

    // pub(crate) fn load(range: Range<usize>) {
    //     todo!()
    // }

    // pub(crate) fn drop(range: Range<usize>) {
    //     todo!()
    // }

    // pub(crate) fn insert_unchecked() {
    //     todo!()
    // }

    // pub(crate) fn remove_unchecked() {
    //     todo!()
    // }
}

impl Index<usize> for TeTree {
    type Output = Self;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}
