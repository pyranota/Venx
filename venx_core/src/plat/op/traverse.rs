use spirv_std::glam::UVec3;

use crate::{
    plat::{node::Node, raw_plat::RawPlat},
    utils::l2s,
};

use super::{EntryOpts, LayerOpts};

impl RawPlat {
    /// Traverse through all voxels in world specified in arguments
    /// Algorithm goes from bottom to up, meaning that some voxels can overlap, in that case works recent-right rule.
    /// Return false in callback to drop traversing of subtree
    pub fn traverse<F>(
        &self,
        entry: EntryOpts,
        layer_opts: LayerOpts,
        entry_opts: EntryOpts,
        mut callback: F,
    ) where
        F: FnMut(Props) -> bool,
    {
        // Iterate over all layers and nodes
        self.opts(None, layer_opts, entry_opts, true, |plat, layer, entry| {
            visit_node(
                self,
                layer as usize,
                entry as usize,
                0,
                UVec3::ZERO,
                self.depth,
                &mut callback,
            );
            return None as Option<()>;
        });

        fn visit_node<F>(
            plat: &RawPlat,
            layer: usize,
            idx: usize,
            parent_idx: usize,
            node_position: UVec3,
            level: u8,
            callback: &mut F,
        ) where
            F: FnMut(Props) -> bool,
        {
            let node = plat[layer][idx];

            if !callback(Props {
                position: &node_position,
                parent_idx: &parent_idx,
                node: &node,
                level,
            }) {
                return;
            }
            // Carefull, might be a bug here. Prev: let size = node.size() / 2;
            let size = l2s(level) / 2;

            // Iterate over all children. Order cannot be changed.
            for (i, child_id) in node.children.iter().enumerate() {
                if *child_id != 0 {
                    let child_pos = Node::get_child_position(i as u32) * (size) + node_position;

                    visit_node(
                        plat,
                        layer,
                        *child_id as usize,
                        idx,
                        child_pos,
                        level - 1,
                        callback,
                    );
                }
            }
        }
    }

    pub fn traverse_unpositioned<F>(&self, entry_opts: EntryOpts, layer_opts: LayerOpts, mut f: F)
    where
        F: FnMut(&Node, usize, UVec3) -> bool,
    {
        todo!();
    }
    /// Traversing all nodes on all levels with voxel overlapping
    /// layers can overlap, but voxel within single layer cannot be overlaped
    /// So if you specify a single layer, there are no overlaps
    /// Also region_position is just some value in global space within this region
    pub fn traverse_region<F>(
        &self,
        region_position: UVec3,
        region_level: u8,
        entry: EntryOpts,
        layer: LayerOpts,
        mut f: F,
    ) where
        F: FnMut(&Node, usize, UVec3) -> bool,
    {
        todo!();
    }
}

pub struct Props<'a> {
    pub position: &'a UVec3,
    pub parent_idx: &'a usize,
    pub node: &'a Node,
    pub level: u8,
}

pub struct PropsUnpositioned<'a> {
    pub parent_idx: &'a usize,
    pub node: &'a Node,
    pub level: u8,
}
