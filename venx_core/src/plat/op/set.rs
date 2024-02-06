use crate::plat::raw_plat::RawPlat;

use spirv_std::{
    glam::{uvec3, UVec3},
    macros::debug_printfln,
};
#[macro_use(print)]
use crate::{
    plat::{layer::layer::Layer, node::Node},
    utils::{l2s},
};

impl Layer {
    /// ty 0 is reserved for air and will remove voxel if there is any
    /// you can add any ty if there is no already created entry for it
    /// It will create one
    pub fn set(&mut self, mut pos: UVec3, entry: u32) {
        if entry == 0 {
            return;
        }
        // Identify starting point according to given entry
        let mut idx = self.entry(entry as usize);
        // dbg!(idx, entry);

        let mut size = l2s(self.depth);

        let mut level = self.depth;

        // If given position is out of bound
        if pos.y >= size || pos.x >= size || pos.z >= size {
            return;
        }

        while level > 1 {
            let child_index = Node::get_child_index(pos, level - 1);

            let branch = self[idx];

            let child_id = branch.children[child_index];

            if child_id == 0 {
                let new_child_id = self.allocate_node();
                self[idx].children[child_index] = new_child_id as u32;
                idx = new_child_id;
            } else {
                idx = self[idx].children[child_index] as usize;
            }

            {
                size /= 2;
                level -= 1;
                pos.x %= size;
                pos.y %= size;
                pos.z %= size;
            }
        }
        let child_index = Node::get_child_index(pos, 0);
        self[idx].children[child_index] = 1;
    }
}

#[cfg(test)]
#[cfg(not(feature = "gpu"))]

mod tests {
    extern crate std;

    use spirv_std::glam::uvec3;

    use crate::plat::{node::Node, raw_plat::RawPlat};

    // #[test]
    // fn problematic_set_voxel() {
    //     let mut plat = RawPlat::new(5, 5, 5);
    //     plat[0].set(uvec3(14, 14, 14), 1);
    //     // plat[0].set(uvec3(0, 0, 0), 2);
    //     // plat[0].set(uvec3(5, 15, 5), 3);
    //     plat[0].set(uvec3(0, 10, 0), 1);

    //     let nodes = &plat[0].nodes;

    //     assert_eq!(
    //         nodes[2],
    //         Node {
    //             flag: 0,
    //             children: [3, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );

    //     std::println!("{:?}", &plat[0].entries[0..10]);
    //     std::println!("{:?}", nodes);
    // }

    #[test]
    fn set_voxel() {
        let mut plat = RawPlat::new(2, 2, 2);

        plat[1].set(uvec3(0, 0, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        // Incorrect entry
        plat[1].set(uvec3(0, 1, 0), 0);
        // Out of bound
        plat[1].set(uvec3(0, 7, 0), 1);
        plat[1].set(uvec3(0, 8, 0), 2);

        let nodes = &plat[1].nodes;

        // std::println!("{:?}", &plat[1].entries[0..10]);
        // std::println!("{:?}", nodes);

        assert_eq!(
            nodes[0],
            Node {
                flag: 9,
                children: [0; 8]
            }
        );

        assert_eq!(
            nodes[1],
            Node {
                flag: 3,
                children: [1; 8]
            }
        );

        assert_eq!(
            nodes[2],
            Node {
                flag: 0,
                children: [3, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[3],
            Node {
                flag: 0,
                children: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[4],
            Node {
                flag: 0,
                children: [5, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[5],
            Node {
                flag: 0,
                children: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[6],
            Node {
                flag: -1,
                children: [7, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        // Another layer
        plat[2].set(uvec3(0, 0, 0), 1);
        plat[2].set(uvec3(0, 0, 0), 2);
        // Incorrect entry
        plat[2].set(uvec3(0, 1, 0), 0);
        // Out of bound
        plat[2].set(uvec3(0, 7, 0), 1);
        plat[2].set(uvec3(0, 8, 0), 2);

        let nodes = &plat[2].nodes;

        // std::println!("{:?}", &plat[1].entries[0..10]);
        // std::println!("{:?}", nodes);

        assert_eq!(
            nodes[0],
            Node {
                flag: 9,
                children: [0; 8]
            }
        );

        assert_eq!(
            nodes[1],
            Node {
                flag: 3,
                children: [1; 8]
            }
        );

        assert_eq!(
            nodes[2],
            Node {
                flag: 0,
                children: [3, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[3],
            Node {
                flag: 0,
                children: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[4],
            Node {
                flag: 0,
                children: [5, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[5],
            Node {
                flag: 0,
                children: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[6],
            Node {
                flag: -1,
                children: [7, 0, 0, 0, 0, 0, 0, 0]
            }
        );
    }
}
