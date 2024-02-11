use core::ops::{Range, RangeInclusive};

use spirv_std::glam::UVec3;

use crate::plat::{layer::layer::Layer, raw_plat::RawPlat};

#[derive(Clone, Copy, PartialEq)]
pub enum EntryOpts {
    All,
    Single(u32),
}

pub type LayerOpts = EntryOpts;

impl LayerOpts {
    pub fn to_range(self) -> RangeInclusive<usize> {
        match self {
            EntryOpts::All => 0..=3,
            EntryOpts::Single(layer_idx) => (layer_idx as usize)..=(layer_idx as usize),
        }
    }
}

// #[macro_export]
// macro_rules! match_layer_idx {
//     ($layer_idx:ident, $plat:ident, $entry_opts:ident, $bottom_up:ident, $callback:tt) => {
//         match $layer_idx {
//             0 => opts_layer!($plat, base, $entry_opts, 0, $bottom_up, $callback),
//             1 => opts_layer!($plat, tmp, $entry_opts, 1, $bottom_up, $callback),
//             2 => opts_layer!($plat, schem, $entry_opts, 2, $bottom_up, $callback),
//             3 => opts_layer!($plat, canvas, $entry_opts, 3, $bottom_up, $callback),
//             _ => panic!("You specified the wrong layer"),
//         };
//     };
// }

#[macro_export]
macro_rules! opts_layer {
    ($plat:ident, $layer_idx:ident, $entry_opts:ident, $bottom_up:ident, $callback:tt) => {
        match $entry_opts {
            EntryOpts::All => {
                let entries = &$plat[$layer_idx].entries;
                if $bottom_up {
                    'entries: for entry in 1..(entries.len()) {
                        if $plat[$layer_idx].entries[entry as usize] == 0 {
                            continue 'entries;
                        } else if let Some(t) =
                            $callback($plat, (&$plat[$layer_idx], $layer_idx as u32), entry as u32)
                        {
                            return Some(t);
                        }
                    }
                } else {
                    'entries: for entry in (1..entries.len()).rev() {
                        if $plat[$layer_idx].entries[entry as usize] == 0 {
                            continue 'entries;
                        } else if let Some(t) =
                            $callback($plat, (&$plat[$layer_idx], $layer_idx as u32), entry as u32)
                        {
                            return Some(t);
                        }
                    }
                }
            }
            EntryOpts::Single(entry) => {
                if $plat[$layer_idx].entries[entry as usize] == 0 {
                } else if let Some(t) =
                    $callback($plat, (&$plat[$layer_idx], $layer_idx as u32), entry as u32)
                {
                    return Some(t);
                }
            }
        }
    };
}

impl RawPlat<'_> {
    // TODO: remove RawPlat from callback
    /// The way to iterate over all layers and entries in right order and with maximum performance
    /// If position is None, no optimizations in entries performed
    /// Collback(Plat, LayerIdx, EntryIdx)
    /// If bottom-up is true, than traversing starting from base layer and goes to canvas
    /// In that case there are overlaps.
    /// If bottom-up is false, than traversing from cavas to base. Its faster, but more unsafe. In that case dont forget return Some(()) in time.
    // #[inline]
    pub fn opts<T, C: FnMut(&RawPlat, (&Layer, u32), u32) -> Option<T>>(
        &self,
        // TODO: remove Opt, and use level for indicating instead.
        position: Option<UVec3>,
        // TODO: level: usize,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        bottom_up: bool,
        callback: &mut C,
    ) -> Option<T> {
        if let LayerOpts::Single(layer_idx) = layer_opts {
            let layer_idx = layer_idx as usize;
            opts_layer!(self, layer_idx, entry_opts, bottom_up, callback)
        } else if bottom_up {
            for layer_idx in 0..4 {
                let layer_idx = layer_idx as usize;
                opts_layer!(self, layer_idx, entry_opts, bottom_up, callback)
            }
        } else {
            for layer_idx in (0..4).rev() {
                let layer_idx = layer_idx as usize;
                opts_layer!(self, layer_idx, entry_opts, bottom_up, callback)
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use std::println;

    use alloc::vec;
    use spirv_std::glam::{uvec3, UVec3};

    use crate::plat::{node::Node, raw_plat::RawPlat};

    use super::{EntryOpts, LayerOpts};

    #[test]
    fn test_opts() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            3,
            3,
            3,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        plat[0].set(UVec3::ZERO, 1);
        plat[1].set(UVec3::ZERO, 1);
        plat[1].set(uvec3(0, 1, 0), 2);
        plat[2].set(UVec3::ZERO, 1);

        plat.opts(
            None,
            LayerOpts::Single(2),
            EntryOpts::Single(1),
            false,
            &mut |_plat, (layer, layer_id), entry| {
                assert!(layer == &plat[2]);
                assert!(entry == 1);
                None as Option<()>
            },
        );

        let mut seq = vec![];

        plat.opts(
            None,
            LayerOpts::All,
            EntryOpts::Single(1),
            false,
            &mut |_plat, (layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );

        // println!("{seq:?}");
        assert_eq!(seq, [(2, 1), (1, 1), (0, 1)]);

        let mut seq = vec![];
        plat.opts(
            None,
            LayerOpts::All,
            EntryOpts::Single(1),
            true,
            &mut |_plat, (layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );

        // println!("{:?}", &plat[1].entries[0..10]);
        assert_eq!(seq, [(0, 1), (1, 1), (2, 1)]);

        let mut seq = vec![];

        plat.opts(
            None,
            LayerOpts::Single(1),
            EntryOpts::All,
            false,
            &mut |_plat, (layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );
        // println!("{seq:?}");
        assert_eq!(seq, [(1, 2), (1, 1)]);

        let mut seq = vec![];
        plat.opts(
            None,
            LayerOpts::All,
            EntryOpts::All,
            false,
            &mut |_plat, (layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );
        // println!("{seq:?}");
        assert_eq!(seq, [(2, 1), (1, 2), (1, 1), (0, 1)]);

        let mut seq = vec![];
        plat.opts(
            None,
            LayerOpts::All,
            EntryOpts::All,
            true,
            &mut |_plat, (layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );
        // println!("{seq:?}");
        assert_eq!(seq, [(0, 1), (1, 1), (1, 2), (2, 1)]);
    }

    #[test]
    fn test_opts_2() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            3,
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

        // Overlapping (Canvas)
        plat[1].set(uvec3(0, 0, 0), 1);
        plat[1].set(uvec3(0, 1, 0), 1);
        plat[1].set(uvec3(0, 2, 0), 1);

        // Overlapping above Canvas
        plat[1].set(uvec3(0, 0, 0), 2);
        plat[1].set(uvec3(0, 1, 0), 2);
        plat[1].set(uvec3(0, 2, 0), 2);

        let mut seq = vec![];
        plat.opts(
            None,
            LayerOpts::All,
            EntryOpts::All,
            false,
            &mut |_plat, (_layer, layer_id), entry| {
                seq.push((layer_id, entry));
                None as Option<()>
            },
        );
        // println!("{seq:?}");
        assert_eq!(seq, [(1, 2), (1, 1), (0, 1)]);
    }
}
