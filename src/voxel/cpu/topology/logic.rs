use super::{
    graph::{Branch, Graph},
    level::GLevel,
};

impl Graph {
    // pub fn new(depth: u8) -> Self {
    //     let mut leaf = GBranch::new(0);
    //     leaf.attr_count = 1;
    //     Graph {
    //         head_holder_idx: 0,
    //         segment_level: 5,
    //         compression_level: 11,
    //         depth: depth as u32,
    //         nodes: vec![
    //             GNode::new_branch_from(GBranch::new(depth)), // Root
    //             GNode::new_branch_from(leaf),                // Leaf
    //         ],
    //     }
    // }
    pub fn new(depth: u8) -> Self {
        let mut levels = vec![
            GLevel {
                nodes: Vec::with_capacity(9_000_000),
            };
            depth as usize + 1
        ];

        levels[depth as usize].nodes.push(Branch::default());
        levels[0].nodes.push(Branch::default());

        Graph {
            depth: depth as u32,
            levels,
            empty_head: 0, // Todo handle this shit
            root: 0,
        }
    }
    // pub async fn load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<(Chunk, MeshSize)> {
    //     todo!()
    // }

    // pub async fn turbo_load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<Chunk> {
    //     todo!()
    // }

    // pub async fn set_voxel_cpu() {}

    // pub async fn complete_segment_cpu(&mut self, segment: Vec<Vec<Vec<u32>>>, position: UVec3) {
    //     todo!()
    // }

    // In future should use unused nodes
    pub fn add_branch(&mut self, level: u8, branch: Branch) -> usize {
        self.levels[level as usize].nodes.push(branch);
        self.levels[level as usize].nodes.len() // - 1
    }
    pub fn depth(&self) -> u8 {
        self.depth as u8
    }

    pub fn size(&self) -> u32 {
        1 << (self.depth())
    }
}
