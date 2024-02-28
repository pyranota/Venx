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

#[cfg(feature = "bitcode_support")]
#[cfg(test)]

mod tests {
    extern crate alloc;
    extern crate std;

    use std::dbg;

    use alloc::vec::Vec;
    use rand::thread_rng;
    use spirv_std::glam::uvec3;

    use crate::{
        plat::{node::Node, raw_plat::RawPlat},
        quick_raw_plat,
    };

    // #[test]
    // fn set_voxel() {
    //     quick_raw_plat!(plat, depth 6, len 32, lenrest 32);

    //     plat[1].set(uvec3(0, 0, 0), 1);
    //     plat[1].set(uvec3(0, 0, 0), 2);
    //     // Incorrect entry
    //     plat[1].set(uvec3(0, 1, 0), 0);
    //     // Out of bound
    //     plat[1].set(uvec3(0, 7, 0), 1);
    //     plat[1].set(uvec3(0, 8, 0), 2);

    //     let nodes = &plat[1].nodes;

    //     // std::println!("{:?}", &plat[1].entries[0..10]);
    //     // std::println!("{:?}", nodes);

    //     // assert_eq!(
    //     //     nodes[0],
    //     //     Node {
    //     //         flag: -1,
    //     //         children: [0; 8]
    //     //     }
    //     // );

    //     dbg!(nodes);
    //     dbg!(&plat[1].level_2);

    //     assert_eq!(
    //         nodes[1],
    //         Node {
    //             flag: 0,
    //             children: [2, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[2],
    //         Node {
    //             flag: 0,
    //             children: [4, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[3],
    //         Node {
    //             flag: 0,
    //             children: [5, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[4],
    //         Node {
    //             flag: -3,
    //             children: [1, 3, 2, 6, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[5],
    //         Node {
    //             flag: 0,
    //             children: [1, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[6],
    //         Node {
    //             flag: 0,
    //             children: [7, 0, 8, 0, 0, 0, 0, 0]
    //         }
    //     );
    //     assert_eq!(
    //         nodes[7],
    //         Node {
    //             flag: 0,
    //             children: [2, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );
    // }

    #[test]
    fn set_count() {
        quick_raw_plat!(plat, depth 6, len 6, lenrest 1);
        plat[0].set(uvec3(7, 20, 5), 1);
        assert_eq!(plat[0].free(), 0);
    }

    #[test]
    fn set_mono_chunk_count() {
        /*
            Nodes:
           Free-head - 1
            l6 - 1
            l5 - 1
            l4 - 2 (Fork + Node)
            l3 - 8

           Total: 13

           Nodes L2:
            Free-head - 1
            l2: 8 * 8

           Total: 65

        */
        quick_raw_plat!(plat, depth 6, len 13, len2 65, lenrest 1);

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    plat[0].set(uvec3(x, y, z), 1);
                }
            }
        }

        assert_eq!(plat[0].free(), 0);
        assert_eq!(plat[0].free_l2(), 0);
    }

    #[test]
    fn set_stereo_chunk_count() {
        /*
            Nodes:
           Free-head - 1
            l6 - 1
            l5 - 1
            fork - 16^3 / 4 (1024)
            l4 - 16^3 (4096)
            l3 - 16^3 (4096)

           Total: 9219

           Nodes L2:
            Free-head - 1
            l2 - 16^3 (4096)

           Total: 4097

        */
        quick_raw_plat!(plat, depth 6, len 9219, len2 4097, lenrest 1);

        let mut voxel_id = 1;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    plat[0].set(uvec3(x, y, z), voxel_id);
                    voxel_id += 1;
                }
            }
        }

        assert_eq!(plat[0].free(), 0);
        assert_eq!(plat[0].free_l2(), 0);
    }

    #[test]
    fn set_mono_sparse_chunks_count_deep() {
        /*
            Nodes:
           Free-head - 1
            l8 - 1
            l7 - 8
            l6 - 8^2 (64)
            l5 - 8^3 (512)
            fork - 8^4 (4096)
            l4 - 8^4 (4096)
            l3 - 8^4 (4096)

           Total: 12874

           Nodes L2:
            Free-head - 1
            l2: 16^3 (4096)

           Total: 4097

        */
        quick_raw_plat!(plat, depth 8, len 12874, len2 4097, lenrest 1);

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    // Set UVec3::ZERO voxel in each chunk;
                    plat[0].set(uvec3(x * 16, y * 16, z * 16), 1);
                }
            }
        }
        assert_eq!(plat[0].free(), 0);
        assert_eq!(plat[0].free_l2(), 0);
    }

    #[test]
    fn set_stereo_filled_chunks_count() {
        /*
            Nodes:
            Free-head - 1
            l6 - 1
            l5 - 8
            fork - 8^2 * (16^3 / 4) (2^6 * 2^10 = 2^14 = 65536)
            l4 - 8^2 * 16^3 (2^6 * 2^12 = 2^18 = 262144)
            l3 - 2^18 (262144)

           Total: 589834

           Nodes L2:
            Free-head - 1
            l2: 2^18 (262144)

           Total: 262145

        */
        quick_raw_plat!(plat, depth 6, len 589834, len2 262145, lenrest 1);

        let mut voxel_id = 1;
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    plat[0].set(uvec3(x, y, z), voxel_id);
                    voxel_id += 1;
                }
            }
        }
        assert_eq!(plat[0].free(), 0);
        assert_eq!(plat[0].free_l2(), 0);
    }

    #[test]
    fn set_stereo_filled_chunks_count_shuffled() {
        /*
            Nodes:
            Free-head - 1
            l6 - 1
            l5 - 8
            fork - 8^2 * (16^3 / 4) (2^6 * 2^10 = 2^14 = 65536)
            l4 - 8^2 * 16^3 (2^6 * 2^12 = 2^18 = 262144)
            l3 - 2^18 (262144)

           Total: 589834

           Nodes L2:
            Free-head - 1
            l2: 2^18 (262144)

           Total: 262145

        */
        quick_raw_plat!(plat, depth 6, len 589834, len2 262145, lenrest 1);

        use rand::seq::SliceRandom;
        let mut ids: Vec<u32> = (1..=(64 * 64 * 64)).collect();
        ids.shuffle(&mut thread_rng());

        let mut idx = 0;
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    plat[0].set(uvec3(x, y, z), ids[idx]);
                    idx += 1;
                }
            }
        }
        assert_eq!(plat[0].free(), 0);
        assert_eq!(plat[0].free_l2(), 0);
    }

    #[test]
    fn set_stereo_filled_chunks_count_forks_shuffled() {
        /*
            forks - 8^2 * (16^3 / 4) (2^6 * 2^10 = 2^14 = 65536)
        */
        quick_raw_plat!(plat, depth 6, len 589834, len2 262145, lenrest 1);

        use rand::seq::SliceRandom;
        let mut ids: Vec<u32> = (1..=(64 * 64 * 64)).collect();
        ids.shuffle(&mut thread_rng());

        let mut idx = 0;
        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    plat[0].set(uvec3(x, y, z), ids[idx]);
                    idx += 1;
                }
            }
        }

        let mut fork_count = 0;

        for node in plat[0].nodes.iter() {
            if node.is_fork() {
                fork_count += 1;
            }
        }
        assert_eq!(fork_count, 65536);
    }

    // #[test]
    // fn set_single() {
    //     /*
    //     p:  | 7 | 20 | 5 |

    //         lvl -> ch_idx
    //           6 -> 0
    //           5 -> 2
    //           4 -> 0
    //           3 -> 7
    //           2 -> 1
    //           1 -> 5
    //     */
    //     quick_raw_plat!(plat, depth 6, len 32, lenrest 7);

    //     plat[1].set(uvec3(7, 20, 5), 1);

    //     let nodes = &plat[1].nodes;

    //     // std::println!("{:?}", &plat[1].entries[0..10]);
    //     // std::println!("{:?}", nodes);

    //     // assert_eq!(
    //     //     nodes[0],
    //     //     Node {
    //     //         flag: -1,
    //     //         children: [0; 8]
    //     //     }
    //     // );

    //     dbg!(nodes);
    //     dbg!(&plat[1].level_2);

    //     // assert_eq!(
    //     //     nodes[1],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [2, 0, 0, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[2],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [4, 0, 0, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[3],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [5, 0, 0, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[4],
    //     //     Node {
    //     //         flag: -3,
    //     //         children: [1, 3, 2, 6, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[5],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [1, 0, 0, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[6],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [7, 0, 8, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    //     // assert_eq!(
    //     //     nodes[7],
    //     //     Node {
    //     //         flag: 0,
    //     //         children: [2, 0, 0, 0, 0, 0, 0, 0]
    //     //     }
    //     // );
    // }
}
