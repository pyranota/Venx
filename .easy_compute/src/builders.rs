use wgpu::util::BufferInitDescriptor;

pub struct BufferBuilder<'a> {
    desc: BufferInitDescriptor<'a>,
}

impl<'a> BufferBuilder<'_> {
    pub fn label(mut self, label: &'a str) -> Self {
        //self.desc.label = Some(label);
        self
    }
    pub fn copy(mut self) -> Self {
        //self.desc.label = Some(label);
        self
    }
}
