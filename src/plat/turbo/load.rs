use std::intrinsics::size_of;

use bytemuck::{cast, cast_ref, cast_slice};
use easy_compute::{BindGroupBuilder, BufferRW, ComputePassDescriptor};
use pollster::block_on;
use venx_core::{plat::chunk::chunk::Chunk, utils::Grid};

use crate::plat::interfaces::load::LoadInterface;

use super::gpu_plat::GpuPlat;

impl LoadInterface for GpuPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: usize) -> Box<Chunk> {
        block_on(async {
            // TODO: Make use of push constants for position and lod_level
            let chunk_buffer = self.cs.new_buffer(unsafe {
                // TODO: Use bytemuck or whatever, just to avoid copying while casting to/from bytes + make it more safe
                venx_core::plat::chunk::chunk::any_as_u8_slice(&Chunk::new(
                    position.to_array(),
                    lod_level,
                    5,
                ))
            });

            let chunk_bg = BindGroupBuilder::new()
                .insert(0, true, chunk_buffer.as_entire_binding())
                .build(&self.cs);

            let output_buffer = self.cs.new_staging_buffer(chunk_buffer.size(), true);

            self.cs
                .eval(|encoder| {
                    {
                        let mut cpass =
                            encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
                        cpass.set_pipeline(&self.load_chunk_pl);
                        cpass.set_bind_group(0, &self.raw_plat_bg.bindgroup, &[]);
                        cpass.set_bind_group(1, &self.base_bg.bindgroup, &[]);
                        cpass.set_bind_group(2, &self.tmp_bg.bindgroup, &[]);
                        cpass.set_bind_group(3, &self.schem_bg.bindgroup, &[]);
                        cpass.set_bind_group(4, &self.canvas_bg.bindgroup, &[]);
                        cpass.set_bind_group(5, &chunk_bg.bindgroup, &[]);

                        cpass.dispatch_workgroups(1, 1, 1);
                    }
                    // Metadata
                    encoder.copy_buffer_to_buffer(
                        &chunk_buffer,
                        0,
                        &output_buffer,
                        0,
                        output_buffer.size(),
                    );
                })
                .await;
            let output_bytes: Vec<usize> = output_buffer.read_manual().await;
        });

        todo!()
    }

    fn load_chunks(&self) {
        todo!()
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
}
