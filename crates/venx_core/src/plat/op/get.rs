use spirv_std::glam::{UVec3, Vec3};

use crate::plat::{
    layer::layer::Layer,
    node::{Node, NodeAddr},
    raw_plat::RawPlat,
};

use super::{EntryOpts, LayerOpts};

type Entry_Idx = usize;
type Layer_Idx = Entry_Idx;
type Node_Idx = Layer_Idx;

#[derive(PartialEq, Eq, Hash)]
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

    /// Is there a voxel or not at given position
    /// Slowest operation you should avoid it as much as possible
    pub fn at(&self, position: Vec3, level: usize, entry: EntryOpts, layer: LayerOpts) -> bool {
        // Small optimization
        // With this we should not calculate children indices each run.
        // let path = self.find_path(position: Vec3, 0);

        // self.get_node_pathed(..).is_some()
        todo!()
    }

    // solid_at -> solid_at_specific. Solid at has no more entry and layer
    pub fn solid_at(
        &self,
        position: Vec3,
        level: usize,
        entry: EntryOpts,
        layer: LayerOpts,
    ) -> bool {
        todo!()
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
        plat::{
            layer::layer::Layer,
            node::Node,
            op::{EntryOpts, LayerOpts},
            raw_plat::RawPlat,
        },
        quick_raw_plat,
    };

    #[test]
    fn get_node() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
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
        let mut base = (Box::new([Node::default(); 23_000]), [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut *base.0, &mut base.1),
            (&mut *tmp.0, &mut tmp.1),
            (&mut *schem.0, &mut schem.1),
            (&mut *canvas.0, &mut canvas.1),
        );

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
        let mut base = (Box::new([Node::default(); 23_000]), [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut *base.0, &mut base.1),
            (&mut *tmp.0, &mut tmp.1),
            (&mut *schem.0, &mut schem.1),
            (&mut *canvas.0, &mut canvas.1),
        );

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
    fn get_voxel_id_leveled() {
        quick_raw_plat!(plat, depth 7);
        plat[1].set(uvec3(0, 0, 0), 1);

        assert_eq!(plat.get_node(uvec3(0, 0, 0), 0).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 1).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 2).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 3).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 4).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 5).voxel_id, 1);
        assert_eq!(plat.get_node(uvec3(0, 0, 0), 6).voxel_id, 1);
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
    }
    #[test]
    fn get_node_positions_only() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
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
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
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
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            3,
            3,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
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
