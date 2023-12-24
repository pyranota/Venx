use std::collections::HashMap;

use self::slice::Slice;

mod layer_interface;
mod slice;

#[derive(Debug)]
pub struct VXLayer {
    pub slices: HashMap<usize, Slice>,
    pub depth: u8,
}

impl VXLayer {
    pub fn new(depth: u8) -> Self {
        let mut slices = HashMap::new();
        slices.insert(1, Slice::new(1, depth));

        VXLayer { slices, depth }
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
