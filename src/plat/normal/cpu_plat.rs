use ouroboros::*;

use venx_core::{
    plat::{
        layer::layer::Layer,
        node::Node,
        node_l2::NodeL2,
        raw_plat::{
            LayerIndex::{Base, Canvas, Schem, Tmp},
            RawPlat,
        },
    },
    utils::l2s,
};

use crate::plat::interfaces::PlatInterface;
#[cfg(feature = "turbo")]
use crate::plat::turbo::gpu_plat::GpuPlat;

// #[derive(bitcode::Encode, bitcode::Decode)]
#[self_referencing]
//#[derive(PartialEq, Debug)]
pub struct CpuPlat {
    // Base
    pub(crate) base_nodes: Vec<Node>,
    pub(crate) base_l2: Vec<NodeL2>,

    // Tmp
    pub(crate) tmp_nodes: Vec<Node>,
    pub(crate) tmp_l2: Vec<NodeL2>,

    // Schem
    pub(crate) schem_nodes: Vec<Node>,
    pub(crate) schem_l2: Vec<NodeL2>,

    // Canvas
    pub(crate) canvas_nodes: Vec<Node>,
    pub(crate) canvas_l2: Vec<NodeL2>,
    #[borrows(mut base_nodes, mut base_l2, mut tmp_nodes, mut tmp_l2, mut schem_nodes, mut schem_l2, mut canvas_nodes, mut canvas_l2)]
    #[covariant]
    pub raw_plat: RawPlat<'this>,
}

impl CpuPlat {
    pub(crate) fn new_plat(depth: usize, chunk_level: usize, segment_level: usize) -> Self {
        // let base = (
        //     vec![Node::default(); 3 * (l2s(depth) * l2s(depth)) as usize + 590_000],
        //     vec![NodeL2::default(); 6 * (l2s(depth) * l2s(depth)) as usize + 80_000],
        // );
        let base = (
            vec![Node::default(); (l2s(depth) * l2s(depth)) as usize + 590_000],
            vec![NodeL2::default(); (l2s(depth) * l2s(depth)) as usize + 80_000],
        );
        let tmp = (vec![Node::default(); 8], vec![NodeL2::default(); 2]);
        let (schem, canvas) = (tmp.clone(), tmp.clone());

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas)
    }
    // TMP
    pub(crate) fn _new_plat_with_length(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        len: usize,
    ) -> Self {
        let base = (vec![Node::default(); len], vec![NodeL2::default(); 10]);
        let (tmp, schem, canvas) = (base.clone(), base.clone(), base.clone());

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas)
    }
    /// Create an empty CpuPlat
    pub(crate) fn new_from(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        mut base: (Vec<Node>, Vec<NodeL2>),
        mut tmp: (Vec<Node>, Vec<NodeL2>),
        mut schem: (Vec<Node>, Vec<NodeL2>),
        mut canvas: (Vec<Node>, Vec<NodeL2>),
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
        _chunk_level: usize,
        _segment_level: usize,
        base: (Vec<Node>, Vec<NodeL2>),
        tmp: (Vec<Node>, Vec<NodeL2>),
        schem: (Vec<Node>, Vec<NodeL2>),
        canvas: (Vec<Node>, Vec<NodeL2>),
    ) -> Self {
        CpuPlatBuilder {
            raw_plat_builder: |// Base
                               base_nodes: &mut Vec<Node>,
                               base_l2: &mut Vec<NodeL2>,

                               // Tmp
                               tmp_nodes: &mut Vec<Node>,
                               tmp_l2: &mut Vec<NodeL2>,

                               // Schem
                               schem_nodes: &mut Vec<Node>,
                               schem_l2: &mut Vec<NodeL2>,

                               // Canvas
                               canvas_nodes: &mut Vec<Node>,
                               canvas_l2: &mut Vec<NodeL2>| {
                RawPlat {
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    depth,
                    layers: [
                        Layer {
                            depth,
                            level_2: base_l2,
                            nodes: base_nodes,
                        },
                        Layer {
                            depth,
                            level_2: tmp_l2,
                            nodes: tmp_nodes,
                        },
                        Layer {
                            depth,
                            level_2: schem_l2,
                            nodes: schem_nodes,
                        },
                        Layer {
                            depth,
                            level_2: canvas_l2,
                            nodes: canvas_nodes,
                        },
                    ],
                }
            },
            base_nodes: base.0,
            base_l2: base.1,
            tmp_nodes: tmp.0,
            tmp_l2: tmp.1,
            schem_nodes: schem.0,
            schem_l2: schem.1,
            canvas_nodes: canvas.0,
            canvas_l2: canvas.1,
        }
        .build()
    }

    #[cfg(feature = "turbo")]
    pub(crate) async fn transfer_to_gpu(self) -> GpuPlat {
        //

        let plat = self.borrow_raw_plat();

        // WARNING! Hardcoded values
        GpuPlat::new_from(
            plat.depth,
            5,
            6,
            (plat[Base].nodes.to_vec(), plat[Base].level_2.to_vec()),
            (plat[Tmp].nodes.to_vec(), plat[Tmp].level_2.to_vec()),
            (plat[Schem].nodes.to_vec(), plat[Schem].level_2.to_vec()),
            (plat[Canvas].nodes.to_vec(), plat[Canvas].level_2.to_vec()),
        )
        .await
    }
}

impl PlatInterface for CpuPlat {}
