use super::tetree::TeTree;

impl std::fmt::Debug for TeTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = String::default();

        for node in &self.nodes {
            if let Some(branch) = node.get_branch() {
                fmt += &format!(" {branch:?}");
            }
            if let Some(holder) = node.get_leaf() {
                fmt += &format!(" {holder:?}");
            }
        }
        f.debug_struct("TeTree").field("nodes", &fmt).finish()
    }
}
