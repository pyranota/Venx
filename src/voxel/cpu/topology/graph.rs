use std::mem::ManuallyDrop;

use glam::UVec3;

use super::level::GLevel;

pub type Idx = usize;

#[derive(Debug)]
pub struct Graph {
    pub(crate) depth: u32,
    pub(crate) levels: Vec<GLevel>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Branch {
    // 0 - normal branch, 1 - link to shared, -1 - empty node
    pub ident: i32,
    // If branch is on 1 level, than all children are identified as blocks
    // u32 == u24 // Each layer can be maximum 500mb
    pub children: [u32; 8],
}

#[derive(Debug)]
pub struct GHolder {
    indicator: u32, // if 0 its a link
    next_idx: u32,
    _ghost: u32,
    _phantom: [u32; 8],
}
#[derive(Debug)]
pub struct GBranch {
    pub ref_count: u32,
    pub attr_count: u32,
    pub level: u32,
    pub children: [u32; 8],
}
// impl GNode {
//     pub(crate) fn new_holder_from(holder: GHolder) -> Self {
//         todo!()
//     }
//     pub(crate) fn new_holder() -> Self {
//         GNode {
//             placeholder: ManuallyDrop::new(todo!()),
//         }
//     }
//     pub(crate) fn is_holder(&self) -> bool {
//         if unsafe { self.placeholder.indicator == 0 } {
//             return true;
//         }
//         false
//     }
//     pub(crate) fn get_holder(&self) -> Option<&GHolder> {
//         return if self.is_holder() {
//             let holder = unsafe { &*self.placeholder };
//             Some(holder)
//         } else {
//             None
//         };
//     }
//     pub(crate) fn get_holder_mut(&mut self) -> Option<&mut GHolder> {
//         return if self.is_holder() {
//             let holder = unsafe { &mut *self.placeholder };
//             Some(holder)
//         } else {
//             None
//         };
//     }
//     pub(crate) fn new_branch(level: u8) -> Self {
//         GNode {
//             branch: ManuallyDrop::new(GBranch::new(level)),
//         }
//     }
//     pub(crate) fn new_branch_from(branch: GBranch) -> Self {
//         GNode {
//             branch: ManuallyDrop::new(branch),
//         }
//     }

//     pub(crate) fn is_branch(&self) -> bool {
//         !self.is_holder()
//     }
//     pub(crate) fn get_branch(&self) -> Option<&GBranch> {
//         return if self.is_branch() {
//             let branch = unsafe { &*self.branch };
//             Some(branch)
//         } else {
//             None
//         };
//     }
//     pub(crate) fn get_branch_mut(&mut self) -> Option<&mut GBranch> {
//         return if self.is_branch() {
//             let branch = unsafe { &mut *self.branch };
//             Some(branch)
//         } else {
//             None
//         };
//     }
// }

impl GBranch {
    pub fn get_child_position(i: u32) -> UVec3 {
        UVec3::new(i & 1, (i >> 1) & 1, (i >> 2) & 1)
    }
    pub fn get_child_index(pos: UVec3, level: u8) -> usize {
        let child_size = 1 << level;
        let x = if pos[0] < child_size { 0 } else { 1 };
        let y = if pos[1] < child_size { 0 } else { 1 };
        let z = if pos[2] < child_size { 0 } else { 1 };
        (x + y * 2 + z * 4) as usize
    }
    pub fn new(level: u8) -> Self {
        Self {
            ref_count: 1,
            attr_count: Default::default(),
            children: Default::default(),
            level: level as u32,
        }
    }
    // pub fn size(&self) -> u32 {
    //     1 << self.level() as u32
    // }
    // pub fn level(&self) -> u8 {
    //     self.level as u8
    // }
}

// #[test]
// fn graph_essentials() {
//     dbg!(GBranch::new(0));
//     let mut node = GNode::new_branch_from(GBranch::new(0));
//     assert_eq!(true, node.is_branch());
//     let branch = node.get_branch();
//     dbg!(branch);

//     // set
//     if let Some(branch) = node.get_branch_mut() {
//         branch.children[0] = 100;
//     }
//     let branch = node.get_branch();
//     dbg!(branch);
// }
