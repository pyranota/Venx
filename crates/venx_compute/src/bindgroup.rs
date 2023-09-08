use pollster::FutureExt;
use wgpu::{
    hal::CommandEncoder, include_wgsl, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBindingType, ComputePass, ShaderStages,
};

use crate::{buffer_ext::BufferRW, compute_server::ComputeServer, pipeline::PipelineBuilder};

pub struct BindGroupVenx {
    pub bindgroup: BindGroup,
    pub layout: BindGroupLayout,
}

#[derive(Default)]
pub struct BindGroupBuilder<'a> {
    layout_entries: Vec<BindGroupLayoutEntry>,
    bind_entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self {
        BindGroupBuilder::default()
    }

    pub fn insert(
        mut self,
        bind_index: u32,
        read_only: bool,
        resource: BindingResource<'a>,
    ) -> Self {
        self.bind_entries.push(wgpu::BindGroupEntry {
            binding: bind_index,
            resource,
        });
        self.layout_entries.push(BindGroupLayoutEntry {
            binding: bind_index,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        self
    }

    pub fn build(self, cs: &ComputeServer) -> BindGroupVenx {
        let layout = cs
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &self.layout_entries,
            });
        let group = cs.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &self.bind_entries,
        });

        BindGroupVenx {
            bindgroup: group,
            layout,
        }
    }
}

#[test]
fn test() {
    pollster::block_on(async {
        let cs = ComputeServer::new().await;

        let module = cs.new_module(include_wgsl!("../test.wgsl"));

        let list_buffer = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 5]));

        let output_buffer = cs.new_staging_buffer(4 * 5, true);

        let bg = BindGroupBuilder::new()
            .insert(0, false, list_buffer.as_entire_binding())
            .build(&cs);

        let pipeline = PipelineBuilder::new(&module, "main")
            .for_bindgroup(&bg)
            .build(&cs);

        cs.eval(|encoder| {
            {
                let mut cpass: ComputePass<'_> =
                    encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
                cpass.set_pipeline(&pipeline);
                cpass.set_bind_group(0, &bg.bindgroup, &[]);
                cpass.dispatch_workgroups(1, 1, 1);
            }
            encoder.copy_buffer_to_buffer(&list_buffer, 0, &output_buffer, 0, 4 * 5);
        })
        .await;

        let data: Vec<u32> = output_buffer.read().await;

        output_buffer.unmap();

        dbg!(data);
    });
}

#[test]
fn buffer_resize() {
    pollster::block_on(async {
        let cs = ComputeServer::new().await;

        // let module = cs.new_module(include_wgsl!("../test.wgsl"));

        let list_buffer = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 5]));

        let output_buffer = cs.new_staging_buffer(4 * 5, false);

        let bg = BindGroupBuilder::new()
            .insert(0, false, list_buffer.as_entire_binding())
            .build(&cs);

        {
            let mut data = output_buffer.write().await;

            let a: &mut [u32] = bytemuck::cast_slice_mut(data.as_mut());

            a[1] = 111;

            dbg!(a);
        }
        output_buffer.unmap();

        // let pipeline = PipelineBuilder::new(&module, "main")
        //     .for_bindgroup(&bg)
        //     .build(&cs);

        cs.eval(|encoder| {
            // {
            //     let mut cpass: ComputePass<'_> =
            //         encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            //     cpass.set_pipeline(&pipeline);
            //     cpass.set_bind_group(0, &bg.bindgroup, &[]);
            //     cpass.dispatch_workgroups(1, 1, 1);
            // }
            encoder.copy_buffer_to_buffer(&output_buffer, 0, &list_buffer, 4, output_buffer.size());
        })
        .await;

        //dbg!(data);
    });
}
