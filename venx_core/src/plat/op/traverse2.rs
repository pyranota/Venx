impl Plat{
    pub fn traverse<F>(&self, entry: Entry, layer: Layer, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        todo!();
    }

    pub fn traverse_unpositioned<F>(&self, entry: Entry, layer: Layer, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        todo!();
    }
    /// Traversing all nodes on all levels with voxel overlapping 
    /// layers can overlap, but voxel within single layer cannot be overlaped
    /// So if you specify a single layer, there are no overlaps
    /// Also region_position is just some value in global space within this region
    pub fn traverse_region<F>(&self,  entry: Entry, layer: Layer, region_position: UVec3, region_level: u8, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        todo!();
    }
}

pub struct TrvProps{
    pub position: &'a UVec3,
    pub parent_idx: &'a Idx,
    pub node: &'a Node,
    pub level: u8,
}

pub struct TrvPropsUnpositioned {
    pub parent_idx: &'a Idx,
    pub node: &'a Node,
    pub level: u8,
}