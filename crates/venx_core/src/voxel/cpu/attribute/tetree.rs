use std::mem::ManuallyDrop;

pub struct TeTree {
    nodes: Vec<TNode>,
}
union TNode {
    leaf: ManuallyDrop<TLeaf>,
    branch: ManuallyDrop<TBranch>,
}
#[derive(Debug)]
struct TLeaf {
    count: u32,
    indicator: i32,
    block_id: i32,
    state: i32,
}

#[derive(Debug)]
struct TBranch {
    count: u32,
    children: [i32; 3],
}

impl TNode {
    pub(crate) fn new_leaf() -> Self {
        todo!()
    }
    pub(crate) fn is_leaf(&self) -> bool {
        todo!()
    }
    pub(crate) fn get_leaf(&self) -> Option<&TLeaf> {
        todo!()
    }
    pub(crate) fn get_leaf_mut(&mut self) -> Option<&mut TLeaf> {
        todo!()
    }
    pub(crate) fn new_branch() -> Self {
        todo!()
    }
    pub(crate) fn is_branch(&self) -> bool {
        todo!()
    }
    pub(crate) fn get_branch(&self) -> Option<&TBranch> {
        todo!()
    }
    pub(crate) fn get_branch_mut(&mut self) -> Option<&mut TBranch> {
        todo!()
    }
}

#[test]
fn test_tree() {
    let node = TNode {
        leaf: ManuallyDrop::new(TLeaf {
            count: 45,
            indicator: -1,
            block_id: 11,
            state: 32,
        }),
    };

    unsafe {
        dbg!(&node.branch);
        dbg!(&node.leaf);
    }

    if let Some(leaf) = node.get_leaf() {}
}
