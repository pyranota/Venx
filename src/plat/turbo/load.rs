use std::intrinsics::size_of;

use bytemuck::{cast, cast_ref, cast_slice};
use easy_compute::{BindGroupBuilder, BufferRW, ComputePassDescriptor, PipelineBuilder};
use glam::uvec3;
use log::info;
use pollster::block_on;
use venx_core::{plat::chunk::chunk::Chunk, utils::Grid};

use crate::plat::interfaces::load::LoadInterface;

use super::gpu_plat::GpuPlat;

impl LoadInterface for GpuPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: usize) -> Box<Chunk> {
        block_on(async {
            // TODO: Make use of push constants for position and lod_level
            let chunk = Chunk::new(position.to_array(), lod_level, 5);
            // let (flatten, chunk_meta) = .to_send();
            let chunk_buffer = self.cs.new_buffer(bytemuck::cast_slice(&[chunk]));
            // let chunk_flatten_buffer = self.cs.new_buffer(bytemuck::cast_slice(&flatten));

            let chunk_bg = BindGroupBuilder::new()
                .insert(0, false, chunk_buffer.as_entire_binding())
                .build(&self.cs);

            let output_buffer = self.cs.new_staging_buffer(chunk_buffer.size(), true);

            // Load pipelines
            let load_chunk_pl = PipelineBuilder::new(&self.module, "load_chunk_2")
                .for_bindgroup(&self.base_bg)
                .for_bindgroup(&self.tmp_bg)
                .for_bindgroup(&self.schem_bg)
                .for_bindgroup(&self.canvas_bg)
                .for_bindgroup(&self.raw_plat_bg)
                .for_bindgroup(&chunk_bg)
                .build(&self.cs);

            self.cs
                .eval(|encoder| {
                    {
                        let mut cpass =
                            encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
                        cpass.set_pipeline(&load_chunk_pl);

                        cpass.set_bind_group(0, &self.base_bg.bindgroup, &[]);
                        cpass.set_bind_group(1, &self.tmp_bg.bindgroup, &[]);
                        cpass.set_bind_group(2, &self.schem_bg.bindgroup, &[]);
                        cpass.set_bind_group(3, &self.canvas_bg.bindgroup, &[]);
                        cpass.set_bind_group(4, &self.raw_plat_bg.bindgroup, &[]);
                        cpass.set_bind_group(5, &chunk_bg.bindgroup, &[]);

                        cpass.dispatch_workgroups(1, 1, 1);
                    }
                    //
                    encoder.copy_buffer_to_buffer(
                        &chunk_buffer,
                        0,
                        &output_buffer,
                        0,
                        output_buffer.size(),
                    );
                })
                .await;
            let output: Vec<Chunk> = output_buffer.read_manual().await;

            output_buffer.unmap();

            Box::new(output[0])
        })
    }

    fn overlay_chunk(&self) {
        todo!()
    }

    fn overlay_chunks(&self) {
        todo!()
    }

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> crate::plat::normal::mesh::Mesh {
        todo!()
    }

    fn load_chunks(&self, blank_chunks: Box<Vec<Chunk>>) -> Box<Vec<Chunk>> {
        block_on(async {
            info!("Prepering buffers and pipeline");
            // let (flatten, chunk_meta) = .to_send();
            let chunk_buffer = self.cs.new_buffer(bytemuck::cast_slice(&blank_chunks));
            // let chunk_flatten_buffer = self.cs.new_buffer(bytemuck::cast_slice(&flatten));

            let chunk_bg = BindGroupBuilder::new()
                .insert(0, false, chunk_buffer.as_entire_binding())
                .build(&self.cs);

            let output_buffer = self.cs.new_staging_buffer(chunk_buffer.size(), true);

            // Load pipelines
            let load_chunk_pl = PipelineBuilder::new(&self.module, "load_chunk_2")
                .for_bindgroup(&self.base_bg)
                .for_bindgroup(&self.tmp_bg)
                .for_bindgroup(&self.schem_bg)
                .for_bindgroup(&self.canvas_bg)
                .for_bindgroup(&self.raw_plat_bg)
                .for_bindgroup(&chunk_bg)
                .build(&self.cs);

            self.cs
                .eval(|encoder| {
                    {
                        let mut cpass =
                            encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
                        cpass.set_pipeline(&load_chunk_pl);

                        cpass.set_bind_group(0, &self.base_bg.bindgroup, &[]);
                        cpass.set_bind_group(1, &self.tmp_bg.bindgroup, &[]);
                        cpass.set_bind_group(2, &self.schem_bg.bindgroup, &[]);
                        cpass.set_bind_group(3, &self.canvas_bg.bindgroup, &[]);
                        cpass.set_bind_group(4, &self.raw_plat_bg.bindgroup, &[]);
                        cpass.set_bind_group(5, &chunk_bg.bindgroup, &[]);

                        cpass.dispatch_workgroups(blank_chunks.len() as u32, 1, 1);
                    }
                    //
                    encoder.copy_buffer_to_buffer(
                        &chunk_buffer,
                        0,
                        &output_buffer,
                        0,
                        output_buffer.size(),
                    );
                })
                .await;
            info!("Queue submited");
            let output: Vec<Chunk> = output_buffer.read_manual().await;

            output_buffer.unmap();
            info!("Chunks are copied");

            Box::new(output)
        })
    }
}
