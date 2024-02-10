use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{node::Node, raw_plat::RawPlat},
    utils::l2s,
};

use super::{EntryOpts, LayerOpts};

#[derive(Clone)]
pub struct Props<'a> {
    pub position: &'a Option<UVec3>,
    pub parent_idx: &'a usize,
    pub node: &'a Node,
    pub level: usize,
    pub entry: u32,
}

impl RawPlat<'_> {
    /// Traverse through all voxels in world specified in arguments
    /// Algorithm goes from bottom to up, meaning that some voxels can overlap, in that case works recent-right rule.
    /// Return false in callback to drop traversing of subtree
    pub fn traverse<F>(&self, layer_opts: LayerOpts, entry_opts: EntryOpts, callback: &mut F)
    where
        F: FnMut(Props) -> bool,
    {
        // Iterate over all layers and nodes
        self.opts(
            None,
            layer_opts,
            entry_opts,
            true,
            &mut |plat, (layer, layer_id), entry| {
                layer.traverse(
                    entry,
                    // TODO: do something about this unsafe cringe
                    layer.entries[entry as usize],
                    Some(UVec3::ZERO),
                    self.depth,
                    callback,
                );
                return None as Option<()>;
            },
        );
    }

    pub fn traverse_unpositioned<F>(
        &self,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        callback: &mut F,
    ) where
        F: FnMut(Props) -> bool,
    {
        // Iterate over all layers and nodes
        self.opts(
            None,
            layer_opts,
            entry_opts,
            true,
            &mut |_plat, (layer, layer_id), entry| {
                layer.traverse(entry, entry as usize, None, self.depth, callback);
                return None as Option<()>;
            },
        );
    }

    /// Traversing all nodes on all levels with voxel overlapping
    /// layers and voxels can overlap
    /// So if you specify a single layer, there are no overlaps
    /// Also region_position is just some value in global space within this region
    /// Dont traverse from level == depth, use normal `traverse`
    pub fn traverse_region<F>(
        &self,
        region_position: UVec3,
        region_level: usize,
        entry_opts: EntryOpts,
        layer_opts: LayerOpts,
        callback: &mut F,
    ) where
        F: FnMut(Props) -> bool,
    {
        // TODO: uncom assert
        // assert_ne!(self.depth, region_level);
        // TODO: optimize with level
        self.opts(
            None,
            layer_opts,
            entry_opts,
            true,
            &mut |_plat, (layer, ..), entry| {
                // We need explicitly call it for all specified entries and layers. Otherwise it would find just one node with most priority.

                if let Some(region_node_idx) = layer.get_node(
                    region_position * l2s(region_level),
                    region_level,
                    entry as usize,
                ) {
                    layer.traverse(
                        entry,
                        region_node_idx,
                        Some(uvec3(0, 0, 0)),
                        region_level,
                        callback,
                    )
                }

                None as Option<()>
            },
        );
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use std::println;

    use alloc::{borrow::ToOwned, vec};
    use spirv_std::glam::{uvec3, UVec3};

    use crate::{
        plat::{
            chunk::chunk::Chunk,
            node::Node,
            op::{EntryOpts, LayerOpts},
            raw_plat::RawPlat,
        },
        utils::l2s,
    };

    #[test]
    fn traverse_region() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            6,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );

        // Base
        plat.base.set(uvec3(14, 14, 14), 1);
        plat.base.set(uvec3(0, 0, 0), 2);
        plat.base.set(uvec3(5, 15, 5), 3);
        plat.base.set(uvec3(0, 10, 0), 1);

        // Canvas
        plat.canvas.set(uvec3(15, 15, 15), 1);
        plat.canvas.set(uvec3(0, 0, 0), 2);
        let mut seq = vec![];

        plat.traverse_region(
            UVec3::ZERO,
            5,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |props| {
                if props.level == 0 {
                    seq.push(props.position.clone());
                }
                true
            },
        );
        assert_eq!(
            seq,
            [
                Some(uvec3(0, 10, 0)),
                Some(uvec3(14, 14, 14)),
                Some(uvec3(0, 0, 0)),
                Some(uvec3(5, 15, 5)),
                Some(uvec3(15, 15, 15)),
                Some(uvec3(0, 0, 0))
            ]
        );
    }

    #[test]
    fn traverse() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            5,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        // Base
        plat.base.set(uvec3(14, 14, 14), 1);
        plat.base.set(uvec3(0, 0, 0), 2);
        plat.base.set(uvec3(5, 15, 5), 3);
        plat.base.set(uvec3(0, 10, 0), 1);

        // Canvas
        plat.canvas.set(uvec3(15, 15, 15), 1);
        plat.canvas.set(uvec3(0, 0, 0), 2);

        let mut seq = vec![];

        plat.traverse(LayerOpts::All, EntryOpts::All, &mut |props| {
            if props.level == 0 {
                seq.push(props.position.clone());
            }
            true
        });

        // println!("{seq:?}");

        assert_eq!(
            seq,
            [
                Some(uvec3(0, 10, 0)),
                Some(uvec3(14, 14, 14)),
                Some(uvec3(0, 0, 0)),
                Some(uvec3(5, 15, 5)),
                Some(uvec3(15, 15, 15)),
                Some(uvec3(0, 0, 0))
            ]
        );
    }
}
