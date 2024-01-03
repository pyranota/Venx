use super::VXLayer;

impl VXLayer {
    pub fn merge(&mut self) {
        for (_, slice) in &mut self.slices {
            slice.graph.merge(&mut self.shared);
        }
    }
}
