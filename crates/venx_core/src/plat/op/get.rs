use spirv_std::glam::{UVec3, Vec3};

use crate::{
    l2s,
    plat::{
        layer::layer::{Layer, Lr},
        node::{Node, NodeAddr},
        raw_plat::RawPlat,
    },
};

type Entry_Idx = usize;
type Layer_Idx = Entry_Idx;
type Node_Idx = Layer_Idx;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GetNodeResult {
    /// If 0 its none
    pub voxel_id: usize,
    pub layer_id: usize,
    pub node_idx: usize,
}

impl GetNodeResult {
    #[inline]
    pub fn is_some(&self) -> bool {
        self.node_idx != 0 || self.voxel_id != 0
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
    #[allow(non_snake_case)]
    pub fn Some(voxel_id: usize, layer_id: usize, node_idx: usize) -> Self {
        Self {
            voxel_id,
            layer_id,
            node_idx,
        }
    }

    #[allow(non_snake_case)]
    pub fn None() -> Self {
        Self {
            voxel_id: 0,
            layer_id: 0,
            node_idx: 0,
        }
    }
}

impl RawPlat<'_> {
    /// If Entry is Entry::All, than it will return the most valuable (by voxel-collection) block
    /// Same goes for Layer, if it is Layer::All, it will return the most higher layer
    /// Return (Node_Idx, (Layer, Entry))
    ///  position is global!
    #[inline]
    pub fn get_node(&self, position: UVec3, level: usize) -> GetNodeResult {
        // TODO make binary (take u64 and divide by 3 bits)
        // Small optimization
        // With this we should not calculate children indices each run.
        //let path = self.find_path(position.as_uvec3(), level);
        //todo!()
        //let path = [];

        //let addr = NodeAddr::from_position(position, self.depth, level);

        for layer_idx in (0..4).rev() {
            let res = self[layer_idx].get_node(position, level, None);

            if res.is_some() {
                return res;
            }
        }

        GetNodeResult::None()

        // self.opts(
        //     Some(position),
        //     layer,
        //     entry,
        //     false,
        //     &mut |_plat, (layer, layer_id), entry| {
        //         if let Some(node_idx) = layer.get_node(position, level, entry as usize) {
        //             return Some((node_idx, (layer_id as usize, entry as usize)));
        //         }
        //         None
        //     },
        // )
        // self.get_node_pathed(position, level, entry, layer, &path)
    }

    fn find_path<'a>(&self, mut position: UVec3, to_level: usize) -> &'a [usize] {
        let mut path = [0; 20];
        let mut current_level = self.depth as usize;
        let mut size = self.size();

        while current_level > to_level {
            let child_index = Node::get_child_index(position, current_level - 1);
            path[current_level as usize] = child_index;

            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }
        todo!()
        // &path
    }

    fn get_node_cached(
        &self,
        mut position: UVec3,
        level: usize,
        layer: usize,
        entry: usize,
    ) -> Option<usize> {
        todo!()
    }
    #[inline]
    // TODO: make it return actual block, but not the entry
    pub fn get_voxel(&self, position: UVec3) -> GetNodeResult {
        self.get_node(position, 0)
    }

    // /// Is there a voxel or not at given position
    // /// Slowest operation you should avoid it as much as possible
    // pub fn at(&self, position: Vec3, level: usize, entry: EntryOpts, layer: LayerOpts) -> bool {
    //     // Small optimization
    //     // With this we should not calculate children indices each run.
    //     // let path = self.find_path(position: Vec3, 0);

    //     // self.get_node_pathed(..).is_some()
    //     todo!()
    // }

    // // solid_at -> solid_at_specific. Solid at has no more entry and layer
    // pub fn solid_at(
    //     &self,
    //     position: Vec3,
    //     level: usize,
    //     entry: EntryOpts,
    //     layer: LayerOpts,
    // ) -> bool {
    //     todo!()
    // }
}

impl Lr<'_> {
    pub fn get_node(
        &self,
        mut position: UVec3,
        level: usize,
        voxel_id_opt: Option<usize>,
    ) -> GetNodeResult {
        // TODO: Handle cases with 4th level
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 1;

        if level == self.depth {
            return GetNodeResult::Some(0, 0, 1);
        }

        while current_level > fork_level {
            let child_index = Node::get_child_index(position, current_level - 1);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    return res;
                }
            } else {
                return GetNodeResult::None();
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            if let Some(needed_voxel_id) = voxel_id_opt {
                if props.voxel_id == needed_voxel_id {
                    props.drop = true;
                } else {
                    return;
                }
            }

            let mut size = size;
            let mut position = position.clone();
            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = Node::get_child_index(position, current_level - 1);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == 3 {
                        let node_l2 = self.level_2[idx];
                        {
                            let size = 4;
                            position.x %= size;
                            position.y %= size;
                            position.z %= size;
                        }
                        if node_l2.is_at(position) {
                            found_idx = GetNodeResult::Some(
                                props.voxel_id as usize,
                                // TODO: Let layer store its id
                                0,
                                below_node_idx as usize,
                            );
                        }
                        return;
                    }
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    size /= 2;
                    position %= size;
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx
    }
    pub fn get_node_gpu(
        &self,
        mut position: UVec3,
        level: usize,
        voxel_id_opt: Option<usize>,
    ) -> usize {
        // let addr = &NodeAddr::from_position(position, self.depth, level);
        // self.get_node_with_addr(addr, level, voxel_id_opt)
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            let child_index = Node::get_child_index(position, current_level - 1);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    return 0;
                }
            } else {
                return 0;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            if let Some(needed_voxel_id) = voxel_id_opt {
                if props.voxel_id == needed_voxel_id {
                    props.drop = true;
                } else {
                    return;
                }
            }

            let mut size = size;
            let mut position = position.clone();
            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = Node::get_child_index(position, current_level - 1);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == 3 {
                        let node_l2 = self.level_2[idx];
                        if node_l2.is_at(position) {
                            found_idx = GetNodeResult::Some(
                                props.voxel_id as usize,
                                // TODO: Let layer store its id
                                0,
                                below_node_idx as usize,
                            );
                        }
                        return;
                    }
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    size /= 2;
                    position %= size;
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx.voxel_id
    }

    pub fn get_node_gpu_no_enum(&self, mut position: UVec3, level: usize) -> usize {
        // let addr = &NodeAddr::from_position(position, self.depth, level);
        // self.get_node_with_addr(addr, level, voxel_id_opt)
        //  return 0;
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            if current_level == 6 {
                return 0;
            }

            let child_index = Node::get_child_index(position, current_level - 1);

            let below_node_idx = self.nodes[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    return 0;
                }
            } else {
                return 0;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            let mut size = size;
            let mut position = position.clone();
            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = Node::get_child_index(position, current_level - 1);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    size /= 2;
                    position %= size;
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx.voxel_id
    }

    pub fn get_node_idx_gpu(&self, mut position: UVec3, level: usize) -> usize {
        // let addr = &NodeAddr::from_position(position, self.depth, level);
        // self.get_node_with_addr(addr, level, voxel_id_opt)
        let mut current_level = self.depth as usize;

        let mut size = l2s(self.depth);
        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            let child_index = Node::get_child_index(position, current_level - 1);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    return below_node_idx as usize;
                }
            } else {
                return 0;
            }
            {
                size /= 2;
                position %= size;
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            // if let Some(needed_voxel_id) = voxel_id_opt {
            //     if props.voxel_id == needed_voxel_id {
            //         props.drop = true;
            //     } else {
            //         return;
            //     }
            // }

            let mut size = size;
            let mut position = position.clone();
            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = Node::get_child_index(position, current_level - 1);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    size /= 2;
                    position %= size;
                    current_level -= 1;
                }
            }

            props.drop = true;
        });

        found_idx.node_idx
    }

    pub fn get_node_with_addr(
        &self,
        addr: &NodeAddr,
        level: usize,
        voxel_id_opt: Option<usize>,
    ) -> (usize) {
        todo!();
        let mut current_level = self.depth as usize;

        let mut found_idx = GetNodeResult::None();
        let fork_level = 4;
        let mut idx = 2;

        while current_level > fork_level {
            let child_index = addr.get_idx(current_level);

            let below_node_idx = self[idx].children[child_index];

            if below_node_idx != 0 {
                idx = below_node_idx as usize;

                if current_level == level + 1 {
                    let res = GetNodeResult::Some(
                        0,
                        // TODO: Let layer store its id
                        0,
                        below_node_idx as usize,
                    );
                    // return res;
                    return 0;
                }
            } else {
                return 0;
                // return GetNodeResult::None();
            }
            {
                current_level -= 1;
            }
        }

        self.iter_fork(idx as usize, &mut |props| {
            if let Some(needed_voxel_id) = voxel_id_opt {
                if props.voxel_id == needed_voxel_id {
                    props.drop = true;
                } else {
                    return;
                }
            }

            let mut current_level = current_level;
            let mut idx = props.node_idx;

            while current_level > level {
                let child_index = addr.get_idx(current_level);

                let below_node_idx = self[idx].children[child_index];

                if below_node_idx != 0 {
                    idx = below_node_idx as usize;
                    if current_level == level + 1 {
                        found_idx = GetNodeResult::Some(
                            props.voxel_id as usize,
                            // TODO: Let layer store its id
                            0,
                            below_node_idx as usize,
                        );
                        //found_idx = Some(below_node_idx as usize);
                    }
                } else {
                    return;
                }
                {
                    current_level -= 1;
                }
            }

            props.drop = true;
        });
        return found_idx.voxel_id;
        //found_idx
    }
}
#[cfg(feature = "bitcode_support")]
#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use std::{dbg, println};

    use alloc::{boxed::Box, vec};
    use rand::Rng;
    use spirv_std::glam::uvec3;

    use crate::{
        plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat},
        quick_raw_plat,
    };

    #[test]
    fn get_node() {
        quick_raw_plat!(plat, depth 5);

        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(4, 4, 1), 3);
        plat[1].set(uvec3(1, 6, 5), 666);

        // let nodes = unsafe { &*plat[1].nodes };
        // std::println!("{:?}", plat[1]);
        // std::println!("{:?}", nodes);

        assert!(plat.get_node(uvec3(0, 1, 0), 0,).voxel_id == 1);
        assert!(plat.get_node(uvec3(0, 0, 0), 0,).voxel_id == 2);
        assert!(plat.get_node(uvec3(1, 6, 5), 0,).voxel_id == 666);
        assert!(plat.get_node(uvec3(4, 4, 1), 0,).voxel_id == 3);
    }

    #[test]
    fn full_matrix() {
        quick_raw_plat!(plat, depth 5, len 23_000);

        let mut rng = rand::thread_rng();

        let mtx: [[[u16; 16]; 16]; 16] = rng.gen();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let voxel_id = mtx[x][y][z] as u32 + 1;
                    plat[0].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
                }
            }
        }

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let voxel_id = mtx[x][y][z] as u32 + 1;

                    assert!(
                        plat.get_node(uvec3(x as u32, y as u32, z as u32), 0,)
                            .voxel_id
                            == voxel_id as usize
                    );
                }
            }
        }
    }

    #[test]
    fn get_node_known_voxel_id() {
        quick_raw_plat!(plat, depth 5, len 23_000);

        let mut rng = rand::thread_rng();

        let mtx: [[[u16; 16]; 16]; 16] = rng.gen();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let voxel_id = mtx[x][y][z] as u32 + 1;
                    plat[0].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
                }
            }
        }

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let voxel_id = mtx[x][y][z] as u32 + 1;

                    assert!(
                        plat[0]
                            .get_node(
                                uvec3(x as u32, y as u32, z as u32),
                                0,
                                Some(voxel_id as usize)
                            )
                            .voxel_id
                            == voxel_id as usize
                    );
                }
            }
        }
    }

    // #[test]
    // fn get_voxel_id_leveled() {
    //     quick_raw_plat!(plat, depth 7);
    //     plat[1].set(uvec3(0, 0, 0), 1);

    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 0).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 1).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 2).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 3).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 4).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 5).voxel_id, 1);
    //     assert_eq!(plat.get_node(uvec3(0, 0, 0), 6).voxel_id, 1);
    // }

    #[test]
    fn get_node_above_fork_level() {
        quick_raw_plat!(plat, depth 7);

        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(4, 4, 1), 3);
        plat[1].set(uvec3(1, 6, 5), 4);
        plat[1].set(uvec3(9, 1, 3), 1);
        plat[1].set(uvec3(15, 0, 14), 2);
        plat[1].set(uvec3(8, 4, 18), 3);
        plat[1].set(uvec3(12, 6, 20), 4);

        // let nodes = unsafe { &*plat[1].nodes };
        // std::println!("{:?}", plat[1]);
        // std::println!("{:?}", nodes);

        assert!(plat.get_node(uvec3(0, 0, 0), 3,).is_some());
        assert!(plat.get_node(uvec3(0, 0, 0), 3,).is_some());

        assert!(plat.get_node(uvec3(0, 0, 0), 4,).is_some());
        assert!(plat.get_node(uvec3(0, 0, 0), 4,).is_some());

        assert!(plat.get_node(uvec3(0, 0, 0), 5,).is_some());
        assert!(plat.get_node(uvec3(0, 0, 0), 6,).is_some());

        assert!(plat.get_node(uvec3(32, 32, 32), 5,).is_none());
        assert!(plat.get_node(uvec3(64, 64, 64), 5,).is_none());

        assert!(plat.get_node(uvec3(12, 3, 312), 0,).is_none());
        assert!(plat.get_node(uvec3(24, 4, 4), 0,).is_none());
    }
    #[test]
    fn get_node_positions_only() {
        quick_raw_plat!(plat, depth 5);
        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(4, 4, 1), 3);
        plat[1].set(uvec3(1, 6, 5), 4);
        plat[1].set(uvec3(9, 1, 3), 1);
        plat[1].set(uvec3(15, 0, 14), 2);
        plat[1].set(uvec3(8, 4, 18), 3);
        plat[1].set(uvec3(12, 6, 20), 4);

        // let nodes = unsafe { &*plat[1].nodes };
        // std::println!("{:?}", plat[1]);
        // std::println!("{:?}", nodes);

        assert!(plat.get_node(uvec3(0, 1, 0), 0,).is_some());

        assert!(plat.get_node(uvec3(0, 0, 0), 0,).is_some());

        assert!(plat.get_node(uvec3(1, 6, 5), 0,).is_some());

        assert!(plat.get_node(uvec3(9, 1, 3), 0,).is_some());
        assert!(plat.get_node(uvec3(1, 2, 3), 0,).is_none());

        assert!(plat.get_node(uvec3(4, 4, 1), 0,).is_some());
        assert!(plat.get_node(uvec3(12, 6, 20), 0,).is_some());
    }

    #[test]
    fn get_node_check_non_existing() {
        quick_raw_plat!(plat, depth 5);
        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(4, 4, 1), 3);
        plat[1].set(uvec3(1, 6, 5), 4);
        plat[1].set(uvec3(9, 1, 3), 1);
        plat[1].set(uvec3(15, 0, 14), 2);
        plat[1].set(uvec3(8, 4, 18), 3);
        plat[1].set(uvec3(12, 6, 20), 4);

        // let nodes = unsafe { &*plat[1].nodes };
        // std::println!("{:?}", plat[1]);
        // std::println!("{:?}", nodes);

        assert!(plat.get_node(uvec3(1, 1, 0), 0,).is_none());

        assert!(plat.get_node(uvec3(14, 0, 0), 0,).is_none());

        assert!(plat.get_node(uvec3(11, 16, 15), 0,).is_none());

        assert!(plat.get_node(uvec3(19, 2, 30), 0,).is_none());
        assert!(plat.get_node(uvec3(11, 22, 31), 0,).is_none());

        assert!(plat.get_node(uvec3(14, 4, 21), 0,).is_none());
        assert!(plat.get_node(uvec3(2, 2, 2), 0,).is_none());
    }

    #[test]
    fn get_voxel() {
        use crate::*;
        quick_raw_plat!(plat, depth 5);
        // Base
        plat[0].set(uvec3(0, 0, 0), 1);
        plat[0].set(uvec3(0, 1, 0), 1);
        plat[0].set(uvec3(0, 2, 0), 1);

        // Overlapping (tmp)
        plat[1].set(uvec3(0, 0, 0), 1);
        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 2, 0), 1);

        // Overlapping (Canvas)
        plat[Canvas].set(uvec3(0, 0, 0), 2);
        plat[Canvas].set(uvec3(0, 1, 0), 2);
        plat[Canvas].set(uvec3(0, 2, 0), 2);

        assert_eq!(plat.get_voxel((0, 0, 0).into()).voxel_id, 2);
        assert_eq!(plat.get_voxel((0, 1, 0).into()).voxel_id, 2);
        assert_eq!(plat.get_voxel((0, 2, 0).into()).voxel_id, 2);
    }
}
