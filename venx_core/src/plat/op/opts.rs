use spirv_std::glam::UVec3;

use crate::plat::raw_plat::RawPlat;

#[derive(Clone, Copy)]
pub enum EntryOpts {
    All,
    Single(u32),
}

pub type LayerOpts = EntryOpts;

impl RawPlat<'_> {
    /// The way to iterate over all layers and entries in right order and with maximum performance.
    /// If position is None, no optimizations in entries performed
    /// Stops if returned Some(()))
    /// Collback(Plat, LayerIdx, EntryIdx)
    // #[inline]
    // pub fn opts_mut<T, C: FnMut(&mut RawPlat, u32, u32) -> Option<T>>(
    //     &mut self,
    //     position: Option<UVec3>,
    //     // TODO: level: u8,
    //     layer_opts: LayerOpts,
    //     entry_opts: EntryOpts,
    //     bottom_up: bool,
    //     callback: &mut C,
    // ) -> Option<T> {
    //     match layer_opts {
    //         LayerOpts::All => {
    //             for i in if bottom_up {
    //                 [0, 1, 2, 3]
    //             } else {
    //                 [3, 2, 1, 0]
    //             } {
    //                 let opt = self.opts_mut(
    //                     position,
    //                     LayerOpts::Single(i),
    //                     entry_opts,
    //                     bottom_up,
    //                     callback,
    //                 );
    //                 if opt.is_some() {
    //                     return opt;
    //                 }
    //             }

    //             None
    //         }
    //         LayerOpts::Single(layer) => match entry_opts {
    //             EntryOpts::All => {
    //                 for i in self[layer as usize]
    //                     .get_entries_in_region(position)
    //                     .iter()
    //                     .skip(1)
    //                 {
    //                     // If entry is 0, that means, all following entries are also 0
    //                     if *i == 0 {
    //                         return None;
    //                     }
    //                     let opt = self.opts_mut(
    //                         position,
    //                         LayerOpts::Single(layer),
    //                         EntryOpts::Single(*i as u32),
    //                         bottom_up,
    //                         callback,
    //                     );
    //                     if opt.is_some() {
    //                         return opt;
    //                     }
    //                 }

    //                 None
    //             }
    //             EntryOpts::Single(entry) => {
    //                 // If given entry does not exist
    //                 if self[layer as usize].entries[entry as usize] == 0 {
    //                     return None;
    //                 } else if let Some(t) = callback(self, layer, entry) {
    //                     return Some(t);
    //                 } else {
    //                     return None;
    //                 }
    //             }
    //         },
    //     }
    // }
    // TODO: remove RawPlat from callback
    /// The way to iterate over all layers and entries in right order and with maximum performance
    /// If position is None, no optimizations in entries performed
    /// Collback(Plat, LayerIdx, EntryIdx)
    /// If bottom-up is true, than traversing starting from base layer and goes to canvas
    /// In that case there are overlaps.
    /// If bottom-up is false, than traversing from cavas to base. Its faster, but more unsafe. In that case dont forget return Some(()) in time.
    #[inline]
    pub fn opts<T, C: FnMut(&RawPlat, u32, u32) -> Option<T>>(
        &self,
        // TODO: remove Opt, and use level for indicating instead.
        position: Option<UVec3>,
        // TODO: level: u8,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        bottom_up: bool,
        callback: &mut C,
    ) -> Option<T> {
        match layer_opts {
            LayerOpts::All => {
                for i in if bottom_up {
                    [0, 1, 2, 3]
                } else {
                    [3, 2, 1, 0]
                } {
                    let opt = self.opts(
                        position,
                        LayerOpts::Single(i),
                        entry_opts,
                        bottom_up,
                        callback,
                    );
                    if opt.is_some() {
                        return opt;
                    }
                }

                None
            }
            LayerOpts::Single(layer) => match entry_opts {
                EntryOpts::All => {
                    if !bottom_up {
                        'entries: for (entry, link) in self[layer as usize]
                            .get_entries_in_region(position)
                            .iter()
                            .enumerate()
                            .skip(1)
                            .rev()
                        {
                            if *link == 0 {
                                continue 'entries;
                            }
                            let opt = self.opts(
                                position,
                                LayerOpts::Single(layer),
                                EntryOpts::Single(entry as u32),
                                bottom_up,
                                callback,
                            );
                            if opt.is_some() {
                                return opt;
                            }
                        }
                    } else {
                        'entries: for (entry, link) in self[layer as usize]
                            .get_entries_in_region(position)
                            .iter()
                            .enumerate()
                            .skip(1)
                        {
                            if *link == 0 {
                                continue 'entries;
                            }
                            let opt = self.opts(
                                position,
                                LayerOpts::Single(layer),
                                EntryOpts::Single(entry as u32),
                                bottom_up,
                                callback,
                            );
                            if opt.is_some() {
                                return opt;
                            }
                        }
                    }

                    None
                }
                EntryOpts::Single(entry) => {
                    // If given entry does not exist
                    if self[layer as usize].entries[entry as usize] == 0 {
                        return None;
                    } else if let Some(t) = callback(self, layer, entry) {
                        return Some(t);
                    } else {
                        return None;
                    }
                }
            },
        }
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
            &mut |_plat, layer, entry| {
                assert!(layer == 2);
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
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
            &mut |_plat, layer, entry| {
                seq.push((layer, entry));
                None as Option<()>
            },
        );
        // println!("{seq:?}");
        assert_eq!(seq, [(1, 2), (1, 1), (0, 1)]);
    }
}
