use spirv_std::glam::UVec3;

use crate::{
    l2s,
    plat::{layer::layer::Lr, node::Node, raw_plat::RawPlat},
};

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
    #[inline]
    pub fn get_node(&self, position: UVec3, level: usize) -> GetNodeResult {
        for layer_idx in (0..4).rev() {
            let res = self[layer_idx].get_node(position, level, None);

            if res.is_some() {
                return res;
            }
        }
        GetNodeResult::None()
    }

    #[inline]
    // TODO: make it return actual block, but not the entry
    pub fn get_voxel(&self, position: UVec3) -> GetNodeResult {
        self.get_node(position, 0)
    }
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
}
#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use rand::Rng;
    use spirv_std::glam::uvec3;

    use crate::{
        plat::{layer::layer::Lr, node::Node, raw_plat::RawPlat},
        quick_raw_plat,
    };

    #[test]
    fn get_node() {
        quick_raw_plat!(plat, depth 5);

        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(4, 4, 1), 3);
        plat[1].set(uvec3(1, 6, 5), 666);

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
        plat[Lr::CANVAS].set(uvec3(0, 0, 0), 2);
        plat[Lr::CANVAS].set(uvec3(0, 1, 0), 2);
        plat[Lr::CANVAS].set(uvec3(0, 2, 0), 2);

        assert_eq!(plat.get_voxel((0, 0, 0).into()).voxel_id, 2);
        assert_eq!(plat.get_voxel((0, 1, 0).into()).voxel_id, 2);
        assert_eq!(plat.get_voxel((0, 2, 0).into()).voxel_id, 2);
    }
}
