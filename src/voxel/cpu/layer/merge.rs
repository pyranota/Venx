use super::VXLayer;

impl VXLayer {
    pub fn merge_segment(&mut self) {
        self.graph.merge_segment((0, 0, 0).into(), self.depth - 1);
    }
}
