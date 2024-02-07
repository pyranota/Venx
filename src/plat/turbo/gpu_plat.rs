use easy_compute::{
    include_spirv, BindGroup, BindGroupBuilder, BindGroupVenx, Buffer, BufferRW, ComputePipeline,
    ComputeServer, PipelineBuilder, ShaderModule,
};
use venx_core::plat::{node::Node, raw_plat::RawPlat};

use crate::plat::{interfaces::PlatInterface, normal::cpu_plat::CpuPlat};

pub struct GpuPlat {
    // Meta
    raw_plat_depth: Buffer,
    raw_plat_bg: BindGroupVenx,
    // raw_plat_freezed: Buffer,
    // Base layer
    base_nodes: Buffer,
    base_entries: Buffer,
    base_bg: BindGroupVenx,

    // Tmp layer
    tmp_nodes: Buffer,
    tmp_entries: Buffer,
    tmp_bg: BindGroupVenx,

    // Schem layer
    schem_nodes: Buffer,
    schem_entries: Buffer,
    schem_bg: BindGroupVenx,

    // Canvas layer
    canvas_nodes: Buffer,
    canvas_entries: Buffer,
    canvas_bg: BindGroupVenx,

    // Easy-compute stuff
    cs: ComputeServer,
    module: ShaderModule,

    // Pipelines
    load_chunk_pl: ComputePipeline,
}

impl PlatInterface for GpuPlat {}

impl GpuPlat {
    pub async fn transfer_from_gpu(self) -> CpuPlat {
        // Prepare Staging buffers for copying

        // Metadata
        let raw_plat_depth_stb = self.cs.new_staging_buffer(self.raw_plat_depth.size(), true);

        // Base layer
        let base_nodes_stb = self.cs.new_staging_buffer(self.base_nodes.size(), true);
        let base_entries_stb = self.cs.new_staging_buffer(self.base_entries.size(), true);

        // Tmp layer
        let tmp_nodes_stb = self.cs.new_staging_buffer(self.tmp_nodes.size(), true);
        let tmp_entries_stb = self.cs.new_staging_buffer(self.tmp_entries.size(), true);

        // Schem layer
        let schem_nodes_stb = self.cs.new_staging_buffer(self.schem_nodes.size(), true);
        let schem_entries_stb = self.cs.new_staging_buffer(self.schem_entries.size(), true);

        // Canvas layer
        let canvas_nodes_stb = self.cs.new_staging_buffer(self.canvas_nodes.size(), true);
        let canvas_entries_stb = self.cs.new_staging_buffer(self.canvas_entries.size(), true);

        // Copy from buffers to staging buffers
        self.cs
            .eval(|encoder| {
                // Metadata
                encoder.copy_buffer_to_buffer(
                    &self.raw_plat_depth,
                    0,
                    &raw_plat_depth_stb,
                    0,
                    raw_plat_depth_stb.size(),
                );

                // Base layer copy
                // Nodes
                encoder.copy_buffer_to_buffer(
                    &self.base_nodes,
                    0,
                    &base_nodes_stb,
                    0,
                    base_nodes_stb.size(),
                );
                // Entries
                encoder.copy_buffer_to_buffer(
                    &self.base_entries,
                    0,
                    &base_entries_stb,
                    0,
                    base_entries_stb.size(),
                );

                // Tmp layer copy
                // Nodes
                encoder.copy_buffer_to_buffer(
                    &self.tmp_nodes,
                    0,
                    &tmp_nodes_stb,
                    0,
                    tmp_nodes_stb.size(),
                );
                // Entries
                encoder.copy_buffer_to_buffer(
                    &self.tmp_entries,
                    0,
                    &tmp_entries_stb,
                    0,
                    tmp_entries_stb.size(),
                );

                // Schem layer copy
                // Nodes
                encoder.copy_buffer_to_buffer(
                    &self.schem_nodes,
                    0,
                    &schem_nodes_stb,
                    0,
                    schem_nodes_stb.size(),
                );
                // Entries
                encoder.copy_buffer_to_buffer(
                    &self.schem_entries,
                    0,
                    &schem_entries_stb,
                    0,
                    schem_entries_stb.size(),
                );

                // Canvas layer copy
                // Nodes
                encoder.copy_buffer_to_buffer(
                    &self.canvas_nodes,
                    0,
                    &canvas_nodes_stb,
                    0,
                    canvas_nodes_stb.size(),
                );
                // Entries
                encoder.copy_buffer_to_buffer(
                    &self.canvas_entries,
                    0,
                    &canvas_entries_stb,
                    0,
                    canvas_entries_stb.size(),
                );
            })
            .await;

        // Map and copy

        // Metadata
        let depth: Vec<u8> = raw_plat_depth_stb.read_manual().await;
        raw_plat_depth_stb.unmap();

        // Base layer
        let base_nodes: Vec<Node> = base_nodes_stb.read_manual().await;
        base_nodes_stb.unmap();
        let base_entries: Vec<usize> = base_entries_stb.read_manual().await;
        base_entries_stb.unmap();

        // Tmp layer
        let tmp_nodes: Vec<Node> = tmp_nodes_stb.read_manual().await;
        tmp_nodes_stb.unmap();
        let tmp_entries: Vec<usize> = tmp_entries_stb.read_manual().await;
        tmp_entries_stb.unmap();

        // Schem layer
        let schem_nodes: Vec<Node> = schem_nodes_stb.read_manual().await;
        schem_nodes_stb.unmap();
        let schem_entries: Vec<usize> = schem_entries_stb.read_manual().await;
        schem_entries_stb.unmap();

        // Canvas layer
        let canvas_nodes: Vec<Node> = canvas_nodes_stb.read_manual().await;
        canvas_nodes_stb.unmap();
        let canvas_entries: Vec<usize> = canvas_entries_stb.read_manual().await;
        canvas_entries_stb.unmap();

        // Create CpuPlat from copied
        // WARNING! Hardcoded values
        CpuPlat {
            base_nodes,
            base_entries,
            tmp_nodes,
            tmp_entries,
            schem_nodes,
            schem_entries,
            canvas_nodes,
            canvas_entries,
        }
    }
    pub async fn new_plat(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        // TODO: make more flexible
        let base = (vec![Node::default(); 128], vec![0; 10]);
        let (tmp, schem, canvas) = (base.clone(), base.clone(), base.clone());

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas).await
    }

    pub(crate) async fn new_from(
        depth: u8,
        chunk_level: u8,
        segment_level: u8,
        base: (Vec<Node>, Vec<usize>),
        tmp: (Vec<Node>, Vec<usize>),
        schem: (Vec<Node>, Vec<usize>),
        canvas: (Vec<Node>, Vec<usize>),
    ) -> Self {
        let mut cs = ComputeServer::new().await;

        // Allocate buffers

        // Metadata layer
        let raw_plat_depth = cs.new_buffer(bytemuck::cast_slice(&[depth]));
        let raw_plat_bg = BindGroupBuilder::new()
            .insert(0, false, raw_plat_depth.as_entire_binding())
            .build(&cs);

        // Base layer
        let base_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&base.0));
        let base_entries_buffer = cs.new_buffer(bytemuck::cast_slice(&base.1));
        let base_bg = BindGroupBuilder::new()
            .insert(0, false, base_nodes_buffer.as_entire_binding())
            .insert(1, false, base_entries_buffer.as_entire_binding())
            .build(&cs);

        // Tmp layer
        let tmp_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&tmp.0));
        let tmp_entries_buffer = cs.new_buffer(bytemuck::cast_slice(&tmp.1));
        let tmp_bg = BindGroupBuilder::new()
            .insert(0, false, tmp_nodes_buffer.as_entire_binding())
            .insert(1, false, tmp_entries_buffer.as_entire_binding())
            .build(&cs);

        // Schem layer
        let schem_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&schem.0));
        let schem_entries_buffer = cs.new_buffer(bytemuck::cast_slice(&schem.1));
        let schem_bg = BindGroupBuilder::new()
            .insert(0, false, schem_nodes_buffer.as_entire_binding())
            .insert(1, false, schem_entries_buffer.as_entire_binding())
            .build(&cs);

        // Canvas layer
        let canvas_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&canvas.0));
        let canvas_entries_buffer = cs.new_buffer(bytemuck::cast_slice(&canvas.1));
        let canvas_bg = BindGroupBuilder::new()
            .insert(0, false, canvas_nodes_buffer.as_entire_binding())
            .insert(1, false, canvas_entries_buffer.as_entire_binding())
            .build(&cs);

        // Load shaders
        let module = cs
            .new_module_spv(include_spirv!(env!("venx_shaders.spv")))
            .unwrap();

        // Load pipelines
        let load_chunk_pl = PipelineBuilder::new(&module, "main")
            .for_bindgroup(&base_bg)
            .for_bindgroup(&tmp_bg)
            .for_bindgroup(&schem_bg)
            .for_bindgroup(&canvas_bg)
            .build(&cs);

        Self {
            base_nodes: base_nodes_buffer,
            base_entries: base_entries_buffer,
            base_bg,
            tmp_nodes: tmp_nodes_buffer,
            tmp_entries: tmp_entries_buffer,
            tmp_bg,
            schem_nodes: schem_nodes_buffer,
            schem_entries: schem_entries_buffer,
            schem_bg,
            canvas_nodes: canvas_nodes_buffer,
            canvas_entries: canvas_entries_buffer,
            canvas_bg,
            cs,
            module,
            load_chunk_pl,
            raw_plat_depth,
            raw_plat_bg,
        }
    }
}
