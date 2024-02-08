use easy_compute::{BindGroupBuilder, BufferRW, ComputeServer, PipelineBuilder};
use wgpu::include_spirv;

fn main() {
    pollster::block_on(test());
}

async fn test() -> anyhow::Result<()> {
    let mut cs = ComputeServer::new().await;
    //const SHADER: &[u8] = include_bytes!(env!("simple_rust_gpu.spv"));

    let module = cs.new_module_spv(include_spirv!(env!("simple_rust_gpu.spv")))?;

    let list_buffer = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 32]));

    let output_buffer = cs.new_staging_buffer(4 * 32, true);

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

    output_buffer
        .read(|a: Vec<i32>| {
            dbg!(a);
        })
        .await;

    // / *
    // EvalBuilder::new()
    //     .begin()
    //     .pipeline(&pipeline)
    //     .bind_group(0, &bg)
    //     .workgroups(1, 1, 1)
    //     .finish()
    //     .copy_buffer_to_buffer(&list_buffer, 0, &output_buffer, 0, output_buffer.size())
    //     .build(&cs);
    // // Or

    // cs.eval(&pipeline, &[&bg], (1, 1, 1), |e| {
    //     e.copy_buffer_to_buffer(&list_buffer, 0, &output_buffer, 0, output_buffer.size());
    // });

    // // Or

    // cs.eval()
    //     .set_pipeline(&pipeline)
    //     .set_bind_group(0, &bg)
    //     .workgroups(1, 1, 1)
    //     .encoder(|e| e.copy_buffer_to_buffer(&list_buffer, 0, &output_buffer, 0, output_buffer.size())
    // ).await;

    // */
    return Ok(());
}
