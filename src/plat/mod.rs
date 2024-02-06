use std::usize;

use easy_compute::{
    include_spirv, BindGroupBuilder, BufferRW, ComputePassDescriptor, ComputeServer,
    PipelineBuilder,
};
use glam::{UVec3, Vec3, Vec4};
use venx_core::{
    plat::{chunk::chunk::Chunk, layer::layer::Layer, node::Node, raw_plat::RawPlat},
    utils::Grid,
};

use self::{
    interfaces::{layer::LayerInterface, load::LoadInterface, PlatInterface},
    normal::{cpu_plat::CpuPlat, mesh::Mesh},
    turbo::gpu_plat::GpuPlat,
};

pub mod interfaces;
#[cfg(feature = "mca_converter")]
mod mca_converter;
mod minecraft_blocks;
mod normal;
mod turbo;

pub struct VenxPlat {
    plat: Plat,
}

pub(crate) enum Plat {
    Cpu(CpuPlat),
    #[cfg(feature = "gpu")]
    Gpu(GpuPlat),
}

impl VenxPlat {
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let plat = Plat::Cpu(CpuPlat {
            raw_plat: RawPlat {
                //controller: Controller::new(depth, chunk_level, segment_level),
                position: (0, 0, 0),
                rotation: (0, 0, 0),
                depth,
                base: Layer::new::<1_280_000>(depth),
                tmp: Layer::new::<128_000>(depth),
                schem: Layer::new::<128_000>(depth),
                canvas: Layer::new::<128_000>(depth),
                //chunks: ChunksStorage {},
            },
        });

        VenxPlat { plat: plat }
    }

    /// Depth, chunk_level, segment_level
    pub async fn new_turbo(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        let mut cs = ComputeServer::new().await;

        let module = cs
            .new_module_spv(include_spirv!(env!("venx_shaders.spv")))
            .unwrap();

        let plat_meta_buffer = cs.new_buffer(bytemuck::cast_slice(&[depth]));

        let base = Layer::new::<1_280_000>(depth);
        let (nodes, meta) = (base.nodes, (base.entries, base.depth));
        let base_buffer = cs.new_buffer(bytemuck::cast_slice(&nodes));

        let output_buffer = cs.new_staging_buffer(base_buffer.size(), true);

        let bg = BindGroupBuilder::new()
            .insert(0, false, base_buffer.as_entire_binding())
            .build(&cs);

        let pipeline = PipelineBuilder::new(&module, "main")
            .for_bindgroup(&bg)
            .build(&cs);

        cs.eval(|encoder| {
            {
                let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
                cpass.set_pipeline(&pipeline);
                cpass.set_bind_group(0, &bg.bindgroup, &[]);
                cpass.dispatch_workgroups(1, 1, 1);
            }
            encoder.copy_buffer_to_buffer(&base_buffer, 0, &output_buffer, 0, output_buffer.size());
        })
        .await;

        output_buffer
            .read(|a: Vec<Node>| {
                for node in a {
                    dbg!(node);
                }
            })
            .await;
        todo!()
        // VenxPlat { plat: plat }
    }
    pub fn transfer_to_gpu(self) -> Self {
        todo!()
    }
    pub fn transfer_to_cpu(self) -> Self {
        todo!()
    }
}

impl PlatInterface for VenxPlat {}

impl LoadInterface for VenxPlat {
    fn load_chunk(&self, position: glam::UVec3, lod_level: u8) -> Chunk {
        match &self.plat {
            Plat::Cpu(ref plat) => plat.load_chunk(position, lod_level),
            Plat::Gpu(ref plat) => plat.load_chunk(position, lod_level),
        }
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

    fn compute_mesh_from_chunk<'a>(&self, chunk: &Chunk) -> Mesh {
        match &self.plat {
            Plat::Cpu(ref plat) => plat.compute_mesh_from_chunk(chunk),
            Plat::Gpu(ref plat) => plat.compute_mesh_from_chunk(chunk),
        }
    }
}

impl LayerInterface for VenxPlat {
    fn set_segment<const SIZE: usize>(
        &mut self,
        layer: usize,
        segment: Grid<SIZE>,
        position: glam::UVec3,
    ) {
        todo!()
    }

    fn set_voxel(&mut self, layer: usize, position: glam::UVec3, ty: usize) {
        match &mut self.plat {
            Plat::Cpu(ref mut plat) => plat.set_voxel(layer, position, ty),
            Plat::Gpu(ref mut plat) => plat.set_voxel(layer, position, ty),
        }
    }

    fn compress(&mut self, layer: usize) {
        todo!()
    }

    fn get_voxel(&self, position: glam::UVec3) -> Option<(usize, usize)> {
        todo!()
    }
}
