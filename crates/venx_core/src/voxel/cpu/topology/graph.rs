use std::mem::ManuallyDrop;

pub struct Graph {
    head_holder_idx: usize,
    segment_size: u32,
    compression_level: u32,
    nodes: Vec<GNode>,
}
union GNode {
    placeholder: ManuallyDrop<GHolder>,
    branch: ManuallyDrop<GBranch>,
}
struct GHolder {
    indicator: u32, // if 0 its a link
    next_idx: usize,
    idx: usize,
}
struct GBranch {
    ref_count: u32,
    attr_count: u32,
    children: [u32; 8],
}
impl GNode {
    pub(crate) fn new_holder() -> Self {
        todo!()
    }
    pub(crate) fn is_holder(&self) -> bool {
        todo!()
    }
    pub(crate) fn get_holder(&self) -> Option<&GHolder> {
        todo!()
    }
    pub(crate) fn get_holder_mut(&mut self) -> Option<&mut GHolder> {
        todo!()
    }
    pub(crate) fn new_branch() -> Self {
        todo!()
    }
    pub(crate) fn is_branch(&self) -> bool {
        todo!()
    }
    pub(crate) fn get_branch(&self) -> Option<&GBranch> {
        todo!()
    }
    pub(crate) fn get_branch_mut(&mut self) -> Option<&mut GBranch> {
        todo!()
    }
}
