use std::collections::HashMap;

use self::slice::Slice;

use super::topology::{
    level::GLevel,
    shared::{LevelCache, Shared},
};

mod get;
mod layer_interface;
mod merge;
mod slice;

#[derive(Debug)]
pub struct VXLayer {
    pub slices: HashMap<usize, Slice>,
    pub shared: Shared,
    pub depth: u8,
}

impl VXLayer {
    pub fn new(depth: u8) -> Self {
        let mut slices = HashMap::new();
        slices.insert(1, Slice::new(1, depth));

        let levels = vec![
            GLevel {
                nodes: Vec::with_capacity(9_000_000),
                empty_head: 0,
            };
            depth as usize + 1
        ];

        let level_caches = vec![
            LevelCache {
                map: HashMap::new()
            };
            depth as usize + 1
        ];
        VXLayer {
            slices,
            depth,
            shared: Shared {
                levels,
                level_caches,
            },
        }
    }
    pub fn get_slice_mut_or_create(&mut self, ty: usize) -> &mut Slice {
        if self.slices.contains_key(&ty) {
            return self.slices.get_mut(&ty).unwrap();
        } else {
            self.slices.insert(ty, Slice::new(ty, self.depth));
            return self.slices.get_mut(&ty).unwrap();
        }

        // if let Some(slice) = self.slices.get_mut(&ty) {
        //     return slice;
        // } else {
        //     todo!()
        // }

        // else {
        //     unsafe {
        //         self.slices.insert(ty, Slice::new(ty, self.depth));
        //         return self.slices.get_mut(&ty).unwrap();
        //     }
        // }
    }
}
