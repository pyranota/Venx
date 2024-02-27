use crate::plat::{node_l2::NodeL2, raw_plat::RawPlat};

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
    pub fn set(&mut self, mut position: UVec3, voxel_id: u32) {
        if voxel_id == 0 {
            return;
        }

        // Root is always on 1 idx;
        let mut idx = 1;

        let mut size = l2s(self.depth);

        let mut level = self.depth;

        assert!(level > 4);

        // If given position is out of bound
        if position.y >= size || position.x >= size || position.z >= size {
            return;
        }

        while level > 3 {
            let child_index = Node::get_child_index(position, level - 1);

            idx = self.set_child(idx, voxel_id, child_index, level, 4);

            {
                size /= 2;
                level -= 1;
                position.x %= size;
                position.y %= size;
                position.z %= size;
            }
        }
        let child_index = Node::get_child_index(position, level - 1);

        let child_node_idx = self[idx][child_index];
        {
            let size = 4;
            position.x %= size;
            position.y %= size;
            position.z %= size;
        }
        if child_node_idx != 0 {
            let node_l2 = &mut self.level_2[child_node_idx as usize];

            node_l2.set(position);
        } else {
            let allocated_node_l2_idx = self.allocate_node::<NodeL2>();
            let node_l2 = &mut self.level_2[allocated_node_l2_idx];

            node_l2.set(position);

            self[idx][child_index] = allocated_node_l2_idx as u32;
        }
        return;
    }
}

// #[cfg(feature = "bitcode_support")]
// #[cfg(test)]

// mod tests {
//     extern crate std;

//     use std::dbg;

//     use spirv_std::glam::uvec3;

//     use crate::{
//         plat::{node::Node, raw_plat::RawPlat},
//         quick_raw_plat,
//     };

//     #[test]
//     fn set_voxel() {
//         quick_raw_plat!(plat, depth 6, len 32, lenrest 32);

//         plat[1].set(uvec3(0, 0, 0), 1);
//         plat[1].set(uvec3(0, 0, 0), 2);
//         // Incorrect entry
//         plat[1].set(uvec3(0, 1, 0), 0);
//         // Out of bound
//         plat[1].set(uvec3(0, 7, 0), 1);
//         plat[1].set(uvec3(0, 8, 0), 2);

//         let nodes = &plat[1].nodes;

//         // std::println!("{:?}", &plat[1].entries[0..10]);
//         // std::println!("{:?}", nodes);

//         // assert_eq!(
//         //     nodes[0],
//         //     Node {
//         //         flag: -1,
//         //         children: [0; 8]
//         //     }
//         // );

//         dbg!(nodes);
//         dbg!(&plat[1].level_2);

//         assert_eq!(
//             nodes[1],
//             Node {
//                 flag: 0,
//                 children: [2, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[2],
//             Node {
//                 flag: 0,
//                 children: [4, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[3],
//             Node {
//                 flag: 0,
//                 children: [5, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[4],
//             Node {
//                 flag: -3,
//                 children: [1, 3, 2, 6, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[5],
//             Node {
//                 flag: 0,
//                 children: [1, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[6],
//             Node {
//                 flag: 0,
//                 children: [7, 0, 8, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[7],
//             Node {
//                 flag: 0,
//                 children: [2, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//     }

//     #[test]
//     fn set_single() {
//         /*
//         p:  | 7 | 20 | 5 |

//             lvl -> ch_idx
//               6 -> 0
//               5 -> 2
//               4 -> 0
//               3 -> 7
//               2 -> 1
//               1 -> 5
//         */
//         quick_raw_plat!(plat, depth 6, len 32, lenrest 7);

//         plat[1].set(uvec3(7, 20, 5), 1);

//         let nodes = &plat[1].nodes;

//         // std::println!("{:?}", &plat[1].entries[0..10]);
//         // std::println!("{:?}", nodes);

//         // assert_eq!(
//         //     nodes[0],
//         //     Node {
//         //         flag: -1,
//         //         children: [0; 8]
//         //     }
//         // );

//         dbg!(nodes);
//         dbg!(&plat[1].level_2);

//         assert_eq!(
//             nodes[1],
//             Node {
//                 flag: 0,
//                 children: [2, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[2],
//             Node {
//                 flag: 0,
//                 children: [4, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[3],
//             Node {
//                 flag: 0,
//                 children: [5, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[4],
//             Node {
//                 flag: -3,
//                 children: [1, 3, 2, 6, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[5],
//             Node {
//                 flag: 0,
//                 children: [1, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[6],
//             Node {
//                 flag: 0,
//                 children: [7, 0, 8, 0, 0, 0, 0, 0]
//             }
//         );
//         assert_eq!(
//             nodes[7],
//             Node {
//                 flag: 0,
//                 children: [2, 0, 0, 0, 0, 0, 0, 0]
//             }
//         );
//     }
// }
