use bytemuck::{NoUninit, Pod, Zeroable};
use easy_compute::{BindGroupBuilder, BindGroupVenx, BufferRW, ComputeServer, PipelineBuilder};
use wgpu::include_spirv;

fn main() {
    pollster::block_on(test());
}

pub struct Plat<'a> {
    pub levels: [Level<'a>; 8],
}

pub struct Level<'a> {
    pub nodes: &'a mut [u32],
}

impl Plat<'_> {
    // pub fn to_binds(&self, cs: &ComputeServer) -> (Vec<easy_compute::Buffer>, BindGroupVenx) {
    //     let mut buffers = vec![];

    //     for level in &self.levels {
    //         buffers.push(cs.new_buffer(bytemuck::cast_slice(level.nodes)))
    //     }

    //     // let output_buffer_level_0 = cs.new_staging_buffer(level_0_buffer.size(), true);

    //     // let output_buffer_level_1 = cs.new_staging_buffer(level_1_buffer.size(), true);

    //     let mut bg = BindGroupBuilder::new();

    //     for (i, buffer) in (&buffers).iter().enumerate() {
    //         bg = bg.insert(i as u32, false, buffer.as_entire_binding());
    //     }

    //     (buffers, bg.build(&cs))
    // }
}

async fn test() -> anyhow::Result<()> {
    let mut cs = ComputeServer::new().await;
    //const SHADER: &[u8] = include_bytes!(env!("simple_rust_gpu.spv"));

    let module = cs.new_module_spv(include_spirv!(env!("simple_rust_gpu.spv")))?;

    let mut plat = Plat {
        levels: [
            Level {
                nodes: &mut [16; 8],
            },
            Level {
                nodes: &mut [32; 3],
            },
            Level {
                nodes: &mut [32333; 33],
            },
            Level {
                nodes: &mut [31112; 31234],
            },
            Level {
                nodes: &mut [16; 8],
            },
            Level {
                nodes: &mut [32; 3],
            },
            Level {
                nodes: &mut [32333; 33],
            },
            Level {
                nodes: &mut [31112; 31234],
            },
        ],
    };

    plat.levels[0].nodes[0] = 78;

    let mut buffers = vec![];

    for level in &plat.levels {
        buffers.push(cs.new_buffer(bytemuck::cast_slice(level.nodes)))
    }

    // let output_buffer_level_0 = cs.new_staging_buffer(level_0_buffer.size(), true);

    // let output_buffer_level_1 = cs.new_staging_buffer(level_1_buffer.size(), true);

    let mut bg = BindGroupBuilder::new();

    for (i, buffer) in (&buffers).iter().enumerate() {
        bg = bg.insert(i as u32, false, buffer.as_entire_binding());
    }
    let mut bg = bg.build(&cs);

    let output_buffer_level_0 = cs.new_staging_buffer(buffers[0].size(), true);

    let pipeline = PipelineBuilder::new(&module, "plats")
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
        encoder.copy_buffer_to_buffer(
            &buffers[0],
            0,
            &output_buffer_level_0,
            0,
            output_buffer_level_0.size(),
        );
    })
    .await;

    output_buffer_level_0
        .read(|a: Vec<i32>| {
            dbg!(a);
        })
        .await;
    // output_buffer_level_1
    //     .read(|a: Vec<i32>| {
    //         dbg!(a);
    //     })
    //     .await;

    return Ok(());
}
