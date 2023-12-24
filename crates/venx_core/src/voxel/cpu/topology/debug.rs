use super::graph::Graph;

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = String::default();

        for node in &self.nodes {
            // if let Some(branch) = node.get_branch() {
            //     fmt += &format!(" {branch:?}");
            // }
            // if let Some(holder) = node.get_holder() {
            //     fmt += &format!(" {holder:?}");
            // }
        }

        f.debug_struct("Graph")
            // .field("head_holder_idx", &self.head_holder_idx)
            // .field("segment_level", &self.segment_level)
            // .field("compression_level", &self.compression_level)
            .field("depth", &self.depth)
            .field("nodes", &fmt)
            .finish()
    }
}
