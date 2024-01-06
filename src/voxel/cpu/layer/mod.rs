use std::collections::HashMap;

use self::slice::Slice;

use super::topology::{
    graph::Graph,
    level::GLevel,
    shared::{LevelCache, Shared},
};

mod get;
mod layer_interface;
mod merge;
mod slice;

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct VXLayer {
    pub graph: Graph,
    pub depth: u8,
}

impl VXLayer {
    pub fn new(depth: u8) -> Self {
        VXLayer {
            graph: Graph::new(depth),
            depth,
        }
    }
    // pub fn get_slice_mut_or_create(&mut self, ty: usize) -> &mut Slice {
    //     if self.slices.contains_key(&ty) {
    //         return self.slices.get_mut(&ty).unwrap();
    //     } else {
    //         self.slices.insert(ty, Slice::new(ty, self.depth));
    //         return self.slices.get_mut(&ty).unwrap();
    //     }

    //     // if let Some(slice) = self.slices.get_mut(&ty) {
    //     //     return slice;
    //     // } else {
    //     //     todo!()
    //     // }

    //     // else {
    //     //     unsafe {
    //     //         self.slices.insert(ty, Slice::new(ty, self.depth));
    //     //         return self.slices.get_mut(&ty).unwrap();
    //     //     }
    //     // }
    // }
}
