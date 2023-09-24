use bytemuck::Pod;
use bytes_cast::{unaligned, BytesCast};
use std::mem::ManuallyDrop;

use crate::voxel::cpu::facade::NewNodeIndex;

#[derive(Clone)]
pub(crate) struct TeTree {
    pub(crate) nodes: Vec<TNode>,
}

// impl BytesCast for TeTree {}

#[derive(Clone, Copy)]
pub(crate) union TNode {
    leaf: ManuallyDrop<TLeaf>,
    branch: ManuallyDrop<TBranch>,
}
unsafe impl BytesCast for TNode {}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TLeaf {
    pub count: u32,
    pub indicator: i32,
    pub block_id: i32,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TBranch {
    pub count: u32,
    pub children: [i32; 3],
}

impl TeTree {
    pub fn new() -> Self {
        TeTree {
            nodes: vec![TNode::new_leaf(0, 0, 0)],
        }
    }
    /// Returns new node index in list
    pub fn add_node(&mut self, node: TNode) -> NewNodeIndex {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
}

impl TNode {
    pub(crate) fn count(&self) -> u32 {
        let leaf = unsafe { &*self.leaf };
        leaf.count
    }
    pub(crate) fn new_leaf(count: u32, block_id: i32, block_state: i32) -> Self {
        TNode {
            leaf: ManuallyDrop::new(TLeaf {
                count,
                indicator: -1,
                block_id,
                state: block_state,
            }),
        }
    }
    pub(crate) fn is_leaf(&self) -> bool {
        if unsafe { self.leaf.indicator == -1 } {
            return true;
        }
        false
    }
    pub(crate) fn get_leaf(&self) -> Option<&TLeaf> {
        return if self.is_leaf() {
            let leaf = unsafe { &*self.leaf };
            Some(leaf)
        } else {
            None
        };
    }
    pub(crate) fn get_leaf_mut(&mut self) -> Option<&mut TLeaf> {
        return if self.is_leaf() {
            let leaf = unsafe { &mut *self.leaf };
            Some(leaf)
        } else {
            None
        };
    }
    pub(crate) fn get_leaf_unchecked(&self) -> &TLeaf {
        unsafe { &*self.leaf }
    }
    pub(crate) fn get_leaf_mut_unchecked(&mut self) -> &mut TLeaf {
        unsafe { &mut *self.leaf }
    }
    pub(crate) fn new_branch(count: u32) -> Self {
        TNode {
            branch: ManuallyDrop::new(TBranch {
                count,
                children: [0; 3],
            }),
        }
    }
    pub(crate) fn set_child(&mut self, inner_idx: usize, child_idx: u32) {
        if let Some(branch) = self.get_branch_mut() {
            branch.children[inner_idx] = child_idx as i32;
        }
    }
    pub(crate) fn is_branch(&self) -> bool {
        !self.is_leaf()
    }
    pub(crate) fn get_branch(&self) -> Option<&TBranch> {
        return if self.is_branch() {
            let branch = unsafe { &*self.branch };
            Some(branch)
        } else {
            None
        };
    }
    pub(crate) fn get_branch_mut(&mut self) -> Option<&mut TBranch> {
        return if self.is_branch() {
            let branch = unsafe { &mut *self.branch };
            Some(branch)
        } else {
            None
        };
    }
    pub(crate) fn get_branch_unchecked(&self) -> &TBranch {
        unsafe { &*self.branch }
    }
    pub(crate) fn get_branch_mut_unchecked(&mut self) -> &mut TBranch {
        unsafe { &mut *self.branch }
    }
}

#[test]
fn test_tree() {
    // Testing branch
    let mut branch = TNode::new_branch(144);
    assert_eq!(branch.is_branch(), true);
    assert_eq!(branch.is_leaf(), false);
    dbg!(branch.get_branch());
    branch.set_child(1, 44);
    dbg!(branch.get_branch());
}
