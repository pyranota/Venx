use wgpu::{include_wgsl, ComputePass};

use crate::{buffer_ext::BufferRW, BindGroupBuilder, ComputeServer, PipelineBuilder};
use anyhow::Result;

#[test]
fn buffer_resize() -> Result<()> {
    pollster::block_on(test())
}

async fn test() -> Result<()> {
    let mut cs = ComputeServer::new().await;

    // let _ = cs.new_module(include_str!("../shaders/mod.wgsl"))?;

    // dbg!(&cs.composer);

    let module = cs.new_module(include_str!("../shaders/simple.wgsl"))?;

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
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bg.bindgroup, &[]);
            cpass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&list_buffer, 0, &output_buffer, 0, output_buffer.size());
    })
    .await;

    dbg!("Hey");

    output_buffer
        .read(|a: Vec<i32>| {
            dbg!(a);
        })
        .await;

    return Ok(());
}
