use ouroboros::*;
use venx_core::plat::{node::Node, raw_plat::RawPlat};

use crate::plat::interfaces::PlatInterface;
// #[derive(bitcode::Encode, bitcode::Decode)]

#[self_referencing]
pub struct CpuPlat {
    // Base
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
    #[borrows(mut base_nodes, mut base_entries, mut tmp_nodes, mut tmp_entries, mut schem_nodes, mut schem_entries, mut canvas_nodes, mut canvas_entries)]
    #[covariant]
    pub raw_plat: RawPlat<'this>,
}

impl CpuPlat {
    pub(crate) fn new_plat(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let base = (vec![Node::default(); 128], vec![0; 10]);
        let (tmp, schem, canvas) = (base.clone(), base.clone(), base.clone());

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
                RawPlat::new(
                    depth,
                    chunk_level,
                    segment_level,
                    (base_nodes, base_entries),
                    (tmp_nodes, tmp_entries),
                    (schem_nodes, schem_entries),
                    (canvas_nodes, canvas_entries),
                )
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
