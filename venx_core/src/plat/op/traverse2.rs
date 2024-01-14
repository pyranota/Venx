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
    pub fn traverse_chunk<F>(&self,  entry: Entry, layer: Layer, chunk_position: UVec3, chunk_level: u8, mut f: F)
    where
        F: FnMut(&GBranch, usize, UVec3) -> bool,
    {
        todo!();
    }
}
