use wgpu::{BindGroupLayout, ComputePipeline, PushConstantRange, ShaderModule};

use crate::{bindgroup::BindGroupVenx, compute_server::ComputeServer};

pub struct PipelineBuilder<'a> {
    entry: &'a str,
    module: &'a ShaderModule,
    label: Option<&'a str>,
    bind_group_layouts: Vec<&'a BindGroupLayout>,
    push_constant_ranges: Vec<&'a PushConstantRange>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(module: &'a ShaderModule, entry: &'a str) -> Self {
        PipelineBuilder {
            entry,
            module,
            label: None,
            bind_group_layouts: vec![],
            push_constant_ranges: vec![],
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn for_bindgroup(mut self, bindgroup: &'a BindGroupVenx) -> Self {
        self.bind_group_layouts.push(&bindgroup.layout);
        self
    }

    pub fn build(self, cs: &ComputeServer) -> ComputePipeline {
        cs.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: self.label,
                layout: Some(
                    &cs.device
                        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: self.label,
                            bind_group_layouts: &self.bind_group_layouts,
                            push_constant_ranges: &[],
                        }),
                ),
                module: self.module,
                entry_point: self.entry,
            })
    }
}
