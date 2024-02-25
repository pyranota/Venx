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

impl Layer<'_> {
    /// ty 0 is reserved for air and will remove voxel if there is any
    /// you can add any ty if there is no already created entry for it
    /// It will create one
    pub fn set(&mut self, mut position: UVec3, voxel_id: u32) {
        if voxel_id == 0 {
            return;
        }

        // Root is always on 2 idx;
        let mut idx = 2;

        let mut size = l2s(self.depth);

        let mut level = self.depth;

        // If given position is out of bound
        if position.y >= size || position.x >= size || position.z >= size {
            return;
        }

        while level > 1 {
            let child_index = Node::get_child_index(position, level - 1);

            idx = self.set_child(idx, voxel_id, child_index, level, 4);
            // let node = &mut self[idx];
            // let child_id = node[child_index];

            {
                size /= 2;
                level -= 1;
                position.x %= size;
                position.y %= size;
                position.z %= size;
            }
        }

        let child_index = Node::get_child_index(position, 0);
        self[idx].children[child_index] = 1;
    }
}

#[cfg(feature = "bitcode_support")]
#[cfg(test)]

mod tests {
    extern crate std;

    use spirv_std::glam::uvec3;

    use crate::{
        plat::{node::Node, raw_plat::RawPlat},
        quick_raw_plat,
    };

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
        quick_raw_plat!(plat, depth 6, len 512);

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

        // assert_eq!(
        //     nodes[0],
        //     Node {
        //         flag: -1,
        //         children: [0; 8]
        //     }
        // );

        assert_eq!(
            nodes[1],
            Node {
                flag: -2,
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
                children: [5, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[4],
            Node {
                flag: 0,
                children: [6, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[5],
            Node {
                flag: -3,
                children: [1, 4, 2, 9, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[6],
            Node {
                flag: 0,
                children: [7, 0, 13, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[7],
            Node {
                flag: 0,
                children: [8, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            nodes[8],
            Node {
                flag: 0,
                children: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        // // Another layer
        // plat[2].set(uvec3(0, 0, 0), 1);
        // plat[2].set(uvec3(0, 0, 0), 2);
        // // Incorrect entry
        // plat[2].set(uvec3(0, 1, 0), 0);
        // // Out of bound
        // plat[2].set(uvec3(0, 7, 0), 1);
        // plat[2].set(uvec3(0, 8, 0), 2);

        // let nodes = &plat[2].nodes;

        // // std::println!("{:?}", &plat[1].entries[0..10]);
        // // std::println!("{:?}", nodes);

        // assert_eq!(
        //     nodes[0],
        //     Node {
        //         flag: 9,
        //         children: [0; 8]
        //     }
        // );

        // assert_eq!(
        //     nodes[1],
        //     Node {
        //         flag: 3,
        //         children: [1; 8]
        //     }
        // );

        // assert_eq!(
        //     nodes[2],
        //     Node {
        //         flag: 0,
        //         children: [3, 0, 0, 0, 0, 0, 0, 0]
        //     }
        // );
        // assert_eq!(
        //     nodes[3],
        //     Node {
        //         flag: 0,
        //         children: [1, 0, 0, 0, 0, 0, 0, 0]
        //     }
        // );
        // assert_eq!(
        //     nodes[4],
        //     Node {
        //         flag: 0,
        //         children: [5, 0, 0, 0, 0, 0, 0, 0]
        //     }
        // );
        // assert_eq!(
        //     nodes[5],
        //     Node {
        //         flag: 0,
        //         children: [1, 0, 0, 0, 0, 0, 0, 0]
        //     }
        // );
        // assert_eq!(
        //     nodes[6],
        //     Node {
        //         flag: -1,
        //         children: [7, 0, 0, 0, 0, 0, 0, 0]
        //     }
        // );
    }
}
