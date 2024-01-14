impl Plat {
        /// ty 0 is reserved for air and will remove voxel if there is any
    /// you can add any ty if there is no already created entry for it
    /// It will create one
    pub fn set(&mut self, mut pos: UVec3, entry: u32, layer: usize) {
        if entry == 0 {
            return;
        }

        // Identify starting point according to given entry
        let mut idx = self.entry(entry as usize);
        // dbg!(idx, entry);

        let mut size = self.size();

        let mut level = self.depth();

        // If given position is out of bound
        if pos.y >= size || pos.x >= size || pos.z >= size {
            return;
        }

        while level > 1 {
            let child_index = Branch::get_child_index(pos, level - 1);

            let branch = &self.levels[level as usize][idx];

            let child_id = branch.children[child_index];

            if child_id == 0 {
                let new_child_id = self.add_branch(level - 1);
                self.levels[level as usize][idx].children[child_index] = new_child_id as u32;
                idx = new_child_id;
            } else {
                idx = self.levels[level as usize][idx].children[child_index] as usize;
            }

            {
                size /= 2;
                level -= 1;
                pos.x %= size;
                pos.y %= size;
                pos.z %= size;
            }
        }
        let child_index = Branch::get_child_index(pos, 0);
        let branch = &mut self.levels[1][idx];
        if entry != 0 {
            branch.children[child_index] = 1;
        } else {
            todo!()
        }
    }
}