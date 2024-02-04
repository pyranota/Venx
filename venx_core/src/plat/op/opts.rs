use spirv_std::glam::UVec3;

use crate::plat::raw_plat::RawPlat;

#[derive(Clone, Copy)]
pub enum EntryOpts {
    All,
    Single(u32),
}

pub type LayerOpts = EntryOpts;

impl RawPlat {
    /// The way to iterate over all layers and entries in right order and with maximum performance.
    /// If position is None, no optimizations in entries performed
    /// Stops if returned Some(()))
    /// Collback(Plat, LayerIdx, EntryIdx)
    #[inline]
    pub fn opts_mut<T, C: FnMut(&mut RawPlat, u32, u32) -> Option<T>>(
        &mut self,
        position: Option<UVec3>,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        bottom_up: bool,
        mut callback: C,
    ) -> Option<T> {
        match layer_opts {
            EntryOpts::All => {
                for i in if bottom_up { 0..=3 } else { 3..=0 } {
                    let opt = self.opts_mut(
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
            EntryOpts::Single(layer) => match entry_opts {
                EntryOpts::All => {
                    for i in self[layer as usize].get_entries_in_region(position) {
                        // If entry is 0, that means, all following entries are also 0
                        if *i == 0 {
                            return None;
                        }
                        let opt = self.opts_mut(
                            position,
                            LayerOpts::Single(layer),
                            EntryOpts::Single(*i as u32),
                            bottom_up,
                            callback,
                        );
                        if opt.is_some() {
                            return opt;
                        }
                    }

                    None
                }
                EntryOpts::Single(entry) => {
                    if let Some(t) = callback(self, layer, entry) {
                        return Some(t);
                    } else {
                        return None;
                    }
                }
            },
        }
    }
    /// The way to iterate over all layers and entries in right order and with maximum performance
    /// If position is None, no optimizations in entries performed
    /// Collback(Plat, LayerIdx, EntryIdx)
    /// If bottom-up is true, than traversing starting from base layer and goes to canvas
    /// In that case there are overlaps.
    /// If bottom-up is false, than traversing from cavas to base. Its faster, but more unsafe. In that case dont forget return Some(()) in time.
    #[inline]
    pub fn opts<T, C: FnMut(&RawPlat, u32, u32) -> Option<T>>(
        &self,
        position: Option<UVec3>,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        bottom_up: bool,
        mut callback: C,
    ) -> Option<T> {
        match layer_opts {
            EntryOpts::All => {
                for i in if bottom_up { 0..=3 } else { 3..=0 } {
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
            EntryOpts::Single(layer) => match entry_opts {
                EntryOpts::All => {
                    for i in self[layer as usize].get_entries_in_region(position) {
                        // If entry is 0, that means, all following entries are also 0
                        if *i == 0 {
                            return None;
                        }
                        let opt = self.opts(
                            position,
                            LayerOpts::Single(layer),
                            EntryOpts::Single(*i as u32),
                            bottom_up,
                            callback,
                        );
                        if opt.is_some() {
                            return opt;
                        }
                    }

                    None
                }
                EntryOpts::Single(entry) => {
                    if let Some(t) = callback(self, layer, entry) {
                        return Some(t);
                    } else {
                        return None;
                    }
                }
            },
        }
    }
}
