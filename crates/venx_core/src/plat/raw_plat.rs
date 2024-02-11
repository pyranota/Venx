use core::ops::{Index, IndexMut};

use spirv_std::glam::UVec3;

use super::{layer::layer::Layer, node::Node};

// TODO: rename to RawPlatMut and create LayerMut
#[derive(PartialEq, Debug)]
pub struct RawPlat<'a> {
    pub position: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
    // pub bcs: Vec<()>,
    // pub texs: Vec<()>,
    // pub sbc: SmallBlockCollection,
    /// Maximal depth of plat, can be extended and/or shrinked
    /// 2^depth represents maximum world size
    pub depth: usize,

    /// Each layer is laying on top of layers behind
    /// To provide cross-game exprience, layers specified
    /// Quick tour of layers and its responsobilities:
    /// 0 - Base layer: Used for simple terrain generation
    /// 1 - Tmp layer: Quick layer for temprorary voxel generated by FWGen
    /// 2 - Schematic: Used to place autopasted schematics, also used for AI buildings provided by FWGen
    /// 3 - Canvas: Each voxel you want to place as a player will go there
    pub base: Layer<'a>,
    pub tmp: Layer<'a>,
    pub schem: Layer<'a>,
    pub canvas: Layer<'a>,
}

impl<'a> RawPlat<'a> {
    pub fn new(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        base: (&'a mut [Node], &'a mut [usize]),
        tmp: (&'a mut [Node], &'a mut [usize]),
        schem: (&'a mut [Node], &'a mut [usize]),
        canvas: (&'a mut [Node], &'a mut [usize]),
    ) -> Self {
        RawPlat {
            //controller: Controller::new(depth, chunk_level, segment_level),
            position: (0, 0, 0),
            rotation: (0, 0, 0),
            depth,
            base: Layer::new(depth, base.0, base.1),
            tmp: Layer::new(depth, tmp.0, tmp.1),
            schem: Layer::new(depth, schem.0, schem.1),
            canvas: Layer::new(depth, canvas.0, canvas.1),
        }
    }
    pub fn layers(&'a self) -> [(&'a str, &'a Layer<'a>); 4] {
        [
            ("base", &self.base),
            ("tmp", &self.tmp),
            ("schem", &self.schem),
            ("canvas", &self.canvas),
        ]
    }
    // #[cfg(test)]
    // pub fn new_test<
    //     const BASE_SIZE: usize,
    //     const TMP_SIZE: usize,
    //     const SCHEM_SIZE: usize,
    //     const CANVAS_SIZE: usize,
    //     const ENTRIES_SIZE: usize,
    // >(
    //     depth: usize,
    //     chunk_level: usize,
    //     segment_level: usize,
    // ) -> (
    //     (
    //         ([Node; BASE_SIZE], [usize; ENTRIES_SIZE]),
    //         ([Node; TMP_SIZE], [usize; ENTRIES_SIZE]),
    //         ([Node; SCHEM_SIZE], [usize; ENTRIES_SIZE]),
    //         ([Node; CANVAS_SIZE], [usize; ENTRIES_SIZE]),
    //     ),
    //     Self,
    // ) {
    //     let mut base = ([Node::default(); BASE_SIZE], [0; ENTRIES_SIZE]);
    //     let mut tmp = ([Node::default(); TMP_SIZE], [0; ENTRIES_SIZE]);
    //     let mut schem = ([Node::default(); SCHEM_SIZE], [0; ENTRIES_SIZE]);
    //     let mut canvas = ([Node::default(); CANVAS_SIZE], [0; ENTRIES_SIZE]);

    //     (
    //         (base, tmp, schem, canvas),
    //         RawPlat {
    //             //controller: Controller::new(depth, chunk_level, segment_level),
    //             position: (0, 0, 0),
    //             rotation: (0, 0, 0),
    //             depth,
    //             base: Layer::new(depth, &mut base.0, &mut base.1),
    //             tmp: Layer::new(depth, &mut tmp.0, &mut tmp.1),
    //             schem: Layer::new(depth, &mut schem.0, &mut schem.1),
    //             canvas: Layer::new(depth, &mut canvas.0, &mut canvas.1),
    //         },
    //     )
    // }
    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    pub fn size(&self) -> u32 {
        1 << (self.depth())
    }
}
// #[cfg(not(feature = "gpu"))]
impl<'a> Index<usize> for RawPlat<'a> {
    type Output = Layer<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.base,
            1 => &self.tmp,
            2 => &self.schem,
            3 => &self.canvas,
            _ => panic!("There is no layer on {} index", index),
        }
    }
}

// #[cfg(not(feature = "gpu"))]
impl<'a> IndexMut<usize> for RawPlat<'a> {
    fn index_mut(&mut self, index_mut: usize) -> &mut Self::Output {
        match index_mut {
            0 => &mut self.base,
            1 => &mut self.tmp,
            2 => &mut self.schem,
            3 => &mut self.canvas,
            _ => panic!("There is no layer on {} index", index_mut),
        }
    }
}
