use std::marker::PhantomPinned;

use ouroboros::*;

use venx_core::{
    plat::{
        layer::layer::Layer,
        node::Node,
        raw_plat::{
            LayerIndex::{Base, Canvas, Schem, Tmp},
            RawPlat,
        },
    },
    utils::l2s,
};

use crate::plat::{interfaces::PlatInterface, turbo::gpu_plat::GpuPlat};

// #[derive(bitcode::Encode, bitcode::Decode)]
#[self_referencing]
//#[derive(PartialEq, Debug)]
pub struct CpuPlat {
    // Base
    pub(crate) base_nodes: Vec<Node>,
    pub(crate) base_entries: Vec<usize>,

    // Tmp
    pub(crate) tmp_nodes: Vec<Node>,
    pub(crate) tmp_entries: Vec<usize>,

    // Schem
    pub(crate) schem_nodes: Vec<Node>,
    pub(crate) schem_entries: Vec<usize>,

    // Canvas
    pub(crate) canvas_nodes: Vec<Node>,
    pub(crate) canvas_entries: Vec<usize>,
    #[borrows(mut base_nodes, mut base_entries, mut tmp_nodes, mut tmp_entries, mut schem_nodes, mut schem_entries, mut canvas_nodes, mut canvas_entries)]
    #[covariant]
    pub raw_plat: RawPlat<'this>,
}

pub struct Test {
    base_nodes: Vec<Node>,
    base_entries: Vec<usize>,

    // Tmp
    tmp_nodes: Vec<Node>,
    tmp_entries: Vec<usize>,

    // Schem
    schem_nodes: Vec<Node>,
    schem_entries: Vec<usize>,

    // Canvas
    canvas_nodes: Vec<Node>,
    canvas_entries: Vec<usize>,

    raw_plat_pointer: *const RawPlat<'static>,

    _pinned: PhantomPinned,
}

impl CpuPlat {
    // pub fn zero_copy_raw_plat<'a>(&'a mut self) -> RawPlat<'a> {
    // RawPlat {
    //     position: (0, 0, 0),
    //     rotation: (0, 0, 0),
    //     depth: 12,
    //     base: Layer {
    //         freezed: false,
    //         depth: 12,
    //         entries: &mut self.base_entries,
    //         nodes: &mut self.base_nodes,
    //     },
    //     tmp: Layer {
    //         freezed: false,
    //         depth: 12,
    //         entries: &mut self.tmp_entries,
    //         nodes: &mut self.tmp_nodes,
    //     },
    //     schem: Layer {
    //         freezed: false,
    //         depth: 12,
    //         entries: &mut self.schem_entries,
    //         nodes: &mut self.schem_nodes,
    //     },
    //     canvas: Layer {
    //         freezed: false,
    //         depth: 12,
    //         entries: &mut self.canvas_entries,
    //         nodes: &mut self.canvas_nodes,
    //     },
    // }
    // }

    pub(crate) fn new_plat(depth: usize, chunk_level: usize, segment_level: usize) -> Self {
        let base = (
            vec![Node::default(); 100 * (l2s(depth) * l2s(depth)) as usize],
            vec![0; 2_200],
        );
        let tmp = (vec![Node::default(); 128_000], vec![0; 1_200]);
        let (schem, canvas) = (tmp.clone(), tmp.clone());

        // let mut tt = Box::pin(Test {
        //     base_nodes: base.0,
        //     base_entries: base.1,
        //     tmp_nodes: tmp.0,
        //     tmp_entries: tmp.1,
        //     schem_nodes: schem.0,
        //     schem_entries: schem.1,
        //     canvas_nodes: canvas.0,
        //     canvas_entries: canvas.1,
        //     raw_plat_pointer: std::ptr::null(),
        //     _pinned: PhantomPinned,
        // });
        // unsafe {
        //     tt.as_mut().get_unchecked_mut().raw_plat_pointer = &mut RawPlat::new(
        //         depth,
        //         chunk_level,
        //         segment_level,
        //         (&mut tt.base_nodes, &mut tt.base_entries),
        //         (&mut tt.tmp_nodes, &mut tt.tmp_entries),
        //         (&mut tt.schem_nodes, &mut tt.schem_entries),
        //         (&mut tt.canvas_nodes, &mut tt.canvas_entries),
        //     )
        // }

        // re.map_mut(
        //     |Test {
        //          base_nodes,
        //          base_entries,
        //          tmp_nodes,
        //          tmp_entries,
        //          schem_nodes,
        //          schem_entries,
        //          canvas_nodes,
        //          canvas_entries,
        //      }| {

        //     },
        // );
        // let a = owning_ref_mut.map_mut(
        //     {

        //     },
        // );

        // BoxRefMut::new(o)

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas)
    }
    // TMP
    pub(crate) fn new_plat_with_length(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        len: usize,
    ) -> Self {
        let base = (vec![Node::default(); len], vec![0; 10]);
        let (tmp, schem, canvas) = (base.clone(), base.clone(), base.clone());

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas)
    }
    /// Create an empty CpuPlat
    pub(crate) fn new_from(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        mut base: (Vec<Node>, Vec<usize>),
        mut tmp: (Vec<Node>, Vec<usize>),
        mut schem: (Vec<Node>, Vec<usize>),
        mut canvas: (Vec<Node>, Vec<usize>),
    ) -> Self {
        // Setup and drop
        Layer::new(0, &mut base.0, &mut base.1);
        Layer::new(0, &mut tmp.0, &mut tmp.1);
        Layer::new(0, &mut schem.0, &mut schem.1);
        Layer::new(0, &mut canvas.0, &mut canvas.1);

        Self::from_existing(depth, chunk_level, segment_level, base, tmp, schem, canvas)
    }
    /// Create CpuPlat with already filled layer components
    pub(crate) fn from_existing(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        base: (Vec<Node>, Vec<usize>),
        tmp: (Vec<Node>, Vec<usize>),
        schem: (Vec<Node>, Vec<usize>),
        canvas: (Vec<Node>, Vec<usize>),
    ) -> Self {
        CpuPlatBuilder {
            raw_plat_builder: |// Base
                               base_nodes: &mut Vec<Node>,
                               base_entries: &mut Vec<usize>,

                               // Tmp
                               tmp_nodes: &mut Vec<Node>,
                               tmp_entries: &mut Vec<usize>,

                               // Schem
                               schem_nodes: &mut Vec<Node>,
                               schem_entries: &mut Vec<usize>,

                               // Canvas
                               canvas_nodes: &mut Vec<Node>,
                               canvas_entries: &mut Vec<usize>| {
                RawPlat {
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    depth,
                    layers: [
                        Layer {
                            freezed: false,
                            depth,
                            entries: base_entries,
                            nodes: base_nodes,
                        },
                        Layer {
                            freezed: false,
                            depth,
                            entries: tmp_entries,
                            nodes: tmp_nodes,
                        },
                        Layer {
                            freezed: false,
                            depth,
                            entries: schem_entries,
                            nodes: schem_nodes,
                        },
                        Layer {
                            freezed: false,
                            depth,
                            entries: canvas_entries,
                            nodes: canvas_nodes,
                        },
                    ],
                }
            },
            base_nodes: base.0,
            base_entries: base.1,
            tmp_nodes: tmp.0,
            tmp_entries: tmp.1,
            schem_nodes: schem.0,
            schem_entries: schem.1,
            canvas_nodes: canvas.0,
            canvas_entries: canvas.1,
        }
        .build()
    }

    pub(crate) async fn transfer_to_gpu(self) -> GpuPlat {
        //

        let plat = self.borrow_raw_plat();

        // WARNING! Hardcoded values
        GpuPlat::new_from(
            plat.depth,
            5,
            6,
            (plat[Base].nodes.to_vec(), plat[Base].entries.to_vec()),
            (plat[Tmp].nodes.to_vec(), plat[Tmp].entries.to_vec()),
            (plat[Schem].nodes.to_vec(), plat[Schem].entries.to_vec()),
            (plat[Canvas].nodes.to_vec(), plat[Canvas].entries.to_vec()),
        )
        .await
    }
}

impl PlatInterface for CpuPlat {}
// #[test]
// fn test_insert_segment() {
//     let mut plat = Plat::new(5, 2, 4);
//     let mut segment = Segment::new(4);
//     segment.set(uvec3(15, 0, 11), 11);

//     plat.insert_segment(segment, uvec3(0, 0, 0));

//     let mut segment = Segment::new(4);
//     segment.set(uvec3(0, 5, 0), 15);

//     plat.insert_segment(segment, uvec3(0, 1, 0));

//     plat.get(0, uvec3(15, 0, 11)).unwrap();
//     plat.get(0, uvec3(0, 16 + 5, 0)).unwrap();
//     assert_eq!(plat.get(0, uvec3(15, 0, 11) + uvec3(0, 16, 0)), None);
//     assert_eq!(plat.get(0, uvec3(0, 0, 0) + uvec3(0, 0, 0)), None);
//     assert_eq!(plat.get(0, uvec3(19, 0, 11) + uvec3(16, 16, 0)), None);
// }
