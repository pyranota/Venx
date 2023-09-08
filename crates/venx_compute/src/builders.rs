use pollster::{block_on, FutureExt};
use wgpu::{
    util::BufferInitDescriptor, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferUsages,
    ShaderStages,
};

use crate::{bindgroup::BindGroupVenx, compute_server::ComputeServer};

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
