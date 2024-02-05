use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{node::Node, raw_plat::RawPlat},
    utils::l2s,
};

use super::{EntryOpts, LayerOpts};

#[derive(Debug, Clone)]
pub struct Props<'a> {
    pub position: &'a Option<UVec3>,
    pub parent_idx: &'a usize,
    pub node: &'a Node,
    pub level: u8,
    pub entry: u32,
}

impl RawPlat {
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
            &mut |plat, layer, entry| {
                self.traverse_from(
                    entry,
                    // TODO: do something about this unsafe cringe
                    plat[layer as usize].entries[entry as usize],
                    Some(UVec3::ZERO),
                    self.depth,
                    layer,
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
            &mut |_plat, layer, entry| {
                self.traverse_from(entry, entry as usize, None, self.depth, layer, callback);
                return None as Option<()>;
            },
        );
    }

    /// Specify `entry` just to forward it to callback's props. It is not used elsewhere
    /// `from_node_position` also just to forward, you can ignore these arguments
    /// To speed up set position to None, but it wont display any position information
    pub fn traverse_from<F>(
        &self,
        entry: u32,
        from_node_idx: usize,
        from_node_position: Option<UVec3>,
        from_level: u8,
        layer: u32,
        mut callback: &mut F,
    ) where
        F: FnMut(Props) -> bool,
    {
        assert_ne!(from_node_idx, 0);

        visit_node(
            self,
            layer as usize,
            from_node_idx,
            0,
            from_node_position,
            from_level,
            entry,
            &mut callback,
        );
        fn visit_node<F>(
            plat: &RawPlat,
            layer: usize,
            idx: usize,
            parent_idx: usize,
            node_position_opt: Option<UVec3>,
            level: u8,
            entry: u32,
            callback: &mut F,
        ) where
            F: FnMut(Props) -> bool,
        {
            let node = plat[layer][idx];

            if !callback(Props {
                position: &node_position_opt,
                parent_idx: &parent_idx,
                node: &node,
                entry,
                level,
            }) || level == 0
            {
                return;
            }
            // WATCH: Careful, might be a bug here. Prev: let size = node.size() / 2;
            let size = l2s(level) / 2;

            // Iterate over all children. Order cannot be changed.
            for (i, child_id) in node.children.iter().enumerate() {
                if *child_id != 0 {
                    // TODO: Profile, it might be slow to handle position this way
                    let mut local_node_position_opt: Option<UVec3> = node_position_opt;
                    if let Some(node_position) = &mut local_node_position_opt {
                        *node_position += Node::get_child_position(i as u32) * size;
                    }

                    visit_node(
                        plat,
                        layer,
                        *child_id as usize,
                        idx,
                        local_node_position_opt,
                        level - 1,
                        entry,
                        callback,
                    );
                }
            }
        }
    }

    /// Traversing all nodes on all levels with voxel overlapping
    /// layers and voxels can overlap
    /// So if you specify a single layer, there are no overlaps
    /// Also region_position is just some value in global space within this region
    /// Dont traverse from level == depth, use normal `traverse`
    pub fn traverse_region<F>(
        &self,
        region_position: UVec3,
        region_level: u8,
        entry_opts: EntryOpts,
        layer_opts: LayerOpts,
        callback: &mut F,
    ) where
        F: FnMut(Props) -> bool,
    {
        assert_ne!(self.depth, region_level);
        // TODO: optimize with level
        self.opts(
            None,
            layer_opts,
            entry_opts,
            true,
            &mut |_plat, layer, entry| {
                // We need explicitly call it for all specified entries and layers. Otherwise it would find just one node with most priority.
                if let Some((region_node_idx, ..)) = self.get_node(
                    region_position * l2s(region_level),
                    region_level,
                    EntryOpts::Single(entry),
                    LayerOpts::Single(layer),
                ) {
                    self.traverse_from(
                        entry,
                        region_node_idx,
                        Some(uvec3(0, 0, 0)),
                        region_level,
                        layer,
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
        let mut plat = RawPlat::new(6, 5, 5);

        // Base
        plat[0].set(uvec3(14, 14, 14), 1);
        plat[0].set(uvec3(0, 0, 0), 2);
        plat[0].set(uvec3(5, 15, 5), 3);
        plat[0].set(uvec3(0, 10, 0), 1);

        // Canvas
        plat[3].set(uvec3(15, 15, 15), 1);
        plat[3].set(uvec3(0, 0, 0), 2);
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
        let mut plat = RawPlat::new(5, 5, 5);

        // Base
        plat[0].set(uvec3(14, 14, 14), 1);
        plat[0].set(uvec3(0, 0, 0), 2);
        plat[0].set(uvec3(5, 15, 5), 3);
        plat[0].set(uvec3(0, 10, 0), 1);

        // Canvas
        plat[3].set(uvec3(15, 15, 15), 1);
        plat[3].set(uvec3(0, 0, 0), 2);

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
