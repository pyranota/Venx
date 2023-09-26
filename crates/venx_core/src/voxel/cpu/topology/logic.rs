use super::graph::{GBranch, GNode, Graph};

impl Graph {
    pub fn new(depth: u8) -> Self {
        let mut leaf = GBranch::new(0);
        leaf.attr_count = 1;
        Graph {
            head_holder_idx: 0,
            segment_level: 5,
            compression_level: 11,
            depth: depth as u32,
            nodes: vec![
                GNode::new_branch_from(GBranch::new(depth)), // Root
                GNode::new_branch_from(leaf),                // Leaf
            ],
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
    pub fn add_branch(&mut self, branch: GBranch) -> usize {
        let node = GNode::new_branch_from(branch);
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    pub fn depth(&self) -> u8 {
        self.depth as u8
    }

    pub fn size(&self) -> u32 {
        1 << (self.depth())
    }
}
