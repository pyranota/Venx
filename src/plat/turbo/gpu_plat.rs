use easy_compute::{
    include_spirv,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupBuilder, BindGroupVenx, Buffer, BufferDescriptor, BufferRW, BufferUsages,
    ComputePipeline, ComputeServer, PipelineBuilder, ShaderModule,
};
use glam::{UVec3, Vec3, Vec4};
use venx_core::plat::{
    chunk::chunk::{Chunk, ChunkLoadRequest},
    node::Node,
    node_l2::NodeL2,
    raw_plat::RawPlat,
};

use crate::plat::{
    interfaces::PlatInterface,
    normal::{
        cpu_plat::CpuPlat,
        mesh::{CHUNK_BUCKET, MESH_SIZE},
    },
};

pub struct GpuPlat {
    // Meta
    pub(crate) raw_plat_depth: Buffer,
    pub(crate) raw_plat_bg: BindGroupVenx,
    // raw_plat_freezed: Buffer,
    // Base layer
    pub(crate) base_nodes: Buffer,
    pub(crate) base_l2: Buffer,
    pub(crate) base_bg: BindGroupVenx,

    // Tmp layer
    pub(crate) tmp_nodes: Buffer,
    pub(crate) tmp_l2: Buffer,
    pub(crate) tmp_bg: BindGroupVenx,

    // Schem layer
    pub(crate) schem_nodes: Buffer,
    pub(crate) schem_l2: Buffer,
    pub(crate) schem_bg: BindGroupVenx,

    // Canvas layer
    pub(crate) canvas_nodes: Buffer,
    pub(crate) canvas_l2: Buffer,
    pub(crate) canvas_bg: BindGroupVenx,

    // Chunks
    pub(crate) chunks_buffer: Buffer,
    pub(crate) chunks_requests_buffer: Buffer,
    pub(crate) chunks_requests_staging_buffer: Buffer,
    pub(crate) chunk_bg: BindGroupVenx,

    // Chunk helpers
    pub(crate) mesh: Buffer,
    pub(crate) mesh_helpers_up: Buffer,
    pub(crate) mesh_helpers_down: Buffer,
    pub(crate) mesh_helpers_left: Buffer,
    pub(crate) mesh_helpers_right: Buffer,
    pub(crate) mesh_helpers_front: Buffer,
    pub(crate) mesh_helpers_back: Buffer,
    pub(crate) mesh_helpers_bg: BindGroupVenx,

    // Easy-compute stuff
    pub(crate) cs: ComputeServer,
    pub(crate) module: ShaderModule,

    // Pipelines
    pub(crate) load_chunk_pl: ComputePipeline,
    pub(crate) to_mesh_greedy_pl: ComputePipeline,
}

impl PlatInterface for GpuPlat {}

impl GpuPlat {
    pub async fn transfer_from_gpu(self) -> CpuPlat {
        // Prepare Staging buffers for copying

        // Metadata
        let raw_plat_depth_stb = self.cs.new_staging_buffer(self.raw_plat_depth.size(), true);
        // Base layer
        let base_nodes_stb = self.cs.new_staging_buffer(self.base_nodes.size(), true);
        let base_l2_stb = self.cs.new_staging_buffer(self.base_l2.size(), true);

        // Tmp layer
        let tmp_nodes_stb = self.cs.new_staging_buffer(self.tmp_nodes.size(), true);
        let tmp_l2_stb = self.cs.new_staging_buffer(self.tmp_l2.size(), true);

        // Schem layer
        let schem_nodes_stb = self.cs.new_staging_buffer(self.schem_nodes.size(), true);
        let schem_l2_stb = self.cs.new_staging_buffer(self.schem_l2.size(), true);

        // Canvas layer
        let canvas_nodes_stb = self.cs.new_staging_buffer(self.canvas_nodes.size(), true);
        let canvas_l2_stb = self.cs.new_staging_buffer(self.canvas_l2.size(), true);

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
                    self.base_nodes.size(),
                );
                // Entries
                encoder.copy_buffer_to_buffer(
                    &self.base_l2,
                    0,
                    &base_l2_stb,
                    0,
                    base_l2_stb.size(),
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
                encoder.copy_buffer_to_buffer(&self.tmp_l2, 0, &tmp_l2_stb, 0, tmp_l2_stb.size());

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
                    &self.schem_l2,
                    0,
                    &schem_l2_stb,
                    0,
                    schem_l2_stb.size(),
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
                    &self.canvas_l2,
                    0,
                    &canvas_l2_stb,
                    0,
                    canvas_l2_stb.size(),
                );
            })
            .await;

        // Map and copy

        // Metadata
        let depth: Vec<usize> = raw_plat_depth_stb.read_manual().await;
        raw_plat_depth_stb.unmap();

        // Base layer
        let base_nodes: Vec<Node> = base_nodes_stb.read_manual().await;
        base_nodes_stb.unmap();
        let base_l2: Vec<NodeL2> = base_l2_stb.read_manual().await;
        base_l2_stb.unmap();

        // Tmp layer
        let tmp_nodes: Vec<Node> = tmp_nodes_stb.read_manual().await;
        tmp_nodes_stb.unmap();
        let tmp_l2: Vec<NodeL2> = tmp_l2_stb.read_manual().await;
        tmp_l2_stb.unmap();

        // Schem layer
        let schem_nodes: Vec<Node> = schem_nodes_stb.read_manual().await;
        schem_nodes_stb.unmap();
        let schem_l2: Vec<NodeL2> = schem_l2_stb.read_manual().await;
        schem_l2_stb.unmap();

        // Canvas layer
        let canvas_nodes: Vec<Node> = canvas_nodes_stb.read_manual().await;
        canvas_nodes_stb.unmap();
        let canvas_l2: Vec<NodeL2> = canvas_l2_stb.read_manual().await;
        canvas_l2_stb.unmap();

        // Create CpuPlat from copied
        // WARNING! Hardcoded values
        CpuPlat::from_existing(
            depth[0],
            5,
            6,
            (base_nodes, base_l2),
            (tmp_nodes, tmp_l2),
            (schem_nodes, schem_l2),
            (canvas_nodes, canvas_l2),
        )
        // CpuPlat::new_from(
        //     depth[0],
        //     5,
        //     6,
        //     (base_nodes, base_l2),
        //     (tmp_nodes, tmp_l2),
        //     (schem_nodes, schem_l2),
        //     (canvas_nodes, canvas_l2),
        // )
    }
    pub async fn new_plat(depth: usize, chunk_level: usize, segment_level: usize) -> Self {
        // TODO: make more flexible
        let base = (vec![Node::default(); 128], vec![NodeL2::default(); 10]);
        let (tmp, schem, canvas) = (base.clone(), base.clone(), base.clone());

        Self::new_from(depth, chunk_level, segment_level, base, tmp, schem, canvas).await
    }

    pub(crate) async fn new_from(
        depth: usize,
        chunk_level: usize,
        segment_level: usize,
        base: (Vec<Node>, Vec<NodeL2>),
        tmp: (Vec<Node>, Vec<NodeL2>),
        schem: (Vec<Node>, Vec<NodeL2>),
        canvas: (Vec<Node>, Vec<NodeL2>),
    ) -> Self {
        let mut cs = ComputeServer::new().await;

        // Allocate buffers

        // Metadata layer
        let raw_plat_depth = cs.new_buffer(bytemuck::cast_slice(&[depth as usize]));
        let raw_plat_bg = BindGroupBuilder::new()
            .insert(0, false, raw_plat_depth.as_entire_binding())
            .build(&cs);

        // Base layer
        let base_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&base.0));
        let base_l2_buffer = cs.new_buffer(bytemuck::cast_slice(&base.1));
        let base_bg = BindGroupBuilder::new()
            .insert(0, false, base_nodes_buffer.as_entire_binding())
            .insert(1, false, base_l2_buffer.as_entire_binding())
            .build(&cs);

        // Tmp layer
        let tmp_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&tmp.0));
        let tmp_l2_buffer = cs.new_buffer(bytemuck::cast_slice(&tmp.1));
        let tmp_bg = BindGroupBuilder::new()
            .insert(0, false, tmp_nodes_buffer.as_entire_binding())
            .insert(1, false, tmp_l2_buffer.as_entire_binding())
            .build(&cs);

        // Schem layer
        let schem_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&schem.0));
        let schem_l2_buffer = cs.new_buffer(bytemuck::cast_slice(&schem.1));
        let schem_bg = BindGroupBuilder::new()
            .insert(0, false, schem_nodes_buffer.as_entire_binding())
            .insert(1, false, schem_l2_buffer.as_entire_binding())
            .build(&cs);

        // Canvas layer
        let canvas_nodes_buffer = cs.new_buffer(bytemuck::cast_slice(&canvas.0));
        let canvas_l2_buffer = cs.new_buffer(bytemuck::cast_slice(&canvas.1));
        let canvas_bg = BindGroupBuilder::new()
            .insert(0, false, canvas_nodes_buffer.as_entire_binding())
            .insert(1, false, canvas_l2_buffer.as_entire_binding())
            .build(&cs);

        // Load shaders
        let module = cs
            .new_module_spv(include_spirv!(env!("venx_shaders.spv")))
            .unwrap();

        let blank_chunks = Box::new(vec![Chunk::new((0, 0, 0), 0, 5); CHUNK_BUCKET]);
        let blank_chunk_requests = Box::new(vec![ChunkLoadRequest::default(); CHUNK_BUCKET]);

        // let chunk_buffer = cs.new_buffer(bytemuck::cast_slice(&blank_chunks));

        let chunk_buffer = cs.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&blank_chunks),
            usage: BufferUsages::STORAGE, //| BufferUsages::COPY_DST,
        });
        let chunk_requests_buffer = cs.new_buffer(bytemuck::cast_slice(&blank_chunk_requests));
        let chunk_requests_staging_buffer =
            cs.new_staging_buffer(chunk_requests_buffer.size(), false);

        // let chunk_requests_staging_buffer = cs.device.create_buffer(&BufferDescriptor {
        //     label: None,
        //     size: chunk_requests_buffer.size(), //size: 32768 * 4 * locs.len() as u64,
        //     usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        //     mapped_at_creation: false,
        // });

        // Fill staging buffer
        // cs.eval(|encoder| {
        //     encoder.copy_buffer_to_buffer(
        //         &chunk_requests_buffer,
        //         0,
        //         &chunk_requests_staging_buffer,
        //         0,
        //         chunk_requests_buffer.size(),
        //     );
        // })
        // .await;

        let chunk_bg = BindGroupBuilder::new()
            .insert(0, false, chunk_buffer.as_entire_binding())
            .insert(1, false, chunk_requests_buffer.as_entire_binding())
            .build(&cs);

        // Load pipelines
        let load_chunk_pl = PipelineBuilder::new(&module, "load_chunk")
            .for_bindgroup(&base_bg)
            .for_bindgroup(&tmp_bg)
            .for_bindgroup(&schem_bg)
            .for_bindgroup(&canvas_bg)
            .for_bindgroup(&raw_plat_bg)
            .for_bindgroup(&chunk_bg)
            .build(&cs);

        let helpers = Box::new(vec![Chunk::new((0, 0, 0), 0, 5); CHUNK_BUCKET]);
        akin::akin! {
            let &orientation = [up, down, back, front, left, right];
            let chunk_helper_~*orientation_buffer = cs.new_buffer(bytemuck::cast_slice(&helpers));
        }

        let mesh = cs.new_buffer(bytemuck::cast_slice(&vec![
            [0; 10];
            MESH_SIZE * CHUNK_BUCKET
        ]));

        let mesh_helper_bg = BindGroupBuilder::new()
            .insert(0, false, mesh.as_entire_binding())
            .insert(1, false, chunk_helper_up_buffer.as_entire_binding())
            .insert(2, false, chunk_helper_down_buffer.as_entire_binding())
            .insert(3, false, chunk_helper_back_buffer.as_entire_binding())
            .insert(4, false, chunk_helper_front_buffer.as_entire_binding())
            .insert(5, false, chunk_helper_left_buffer.as_entire_binding())
            .insert(6, false, chunk_helper_right_buffer.as_entire_binding())
            .build(&cs);

        // Load pipelines
        let to_mesh_greedy_pl = PipelineBuilder::new(&module, "to_mesh_greedy")
            .for_bindgroup(&base_bg)
            .for_bindgroup(&tmp_bg)
            .for_bindgroup(&schem_bg)
            .for_bindgroup(&canvas_bg)
            .for_bindgroup(&raw_plat_bg)
            .for_bindgroup(&chunk_bg)
            .for_bindgroup(&mesh_helper_bg)
            .build(&cs);

        Self {
            base_nodes: base_nodes_buffer,
            base_l2: base_l2_buffer,
            base_bg,
            tmp_nodes: tmp_nodes_buffer,
            tmp_l2: tmp_l2_buffer,
            tmp_bg,
            schem_nodes: schem_nodes_buffer,
            schem_l2: schem_l2_buffer,
            schem_bg,
            canvas_nodes: canvas_nodes_buffer,
            canvas_l2: canvas_l2_buffer,
            canvas_bg,
            cs,
            module,
            load_chunk_pl,
            raw_plat_depth,
            raw_plat_bg,
            chunks_buffer: chunk_buffer,
            chunk_bg,
            mesh_helpers_up: chunk_helper_up_buffer,
            mesh_helpers_down: chunk_helper_down_buffer,
            mesh_helpers_left: chunk_helper_left_buffer,
            mesh_helpers_right: chunk_helper_right_buffer,
            mesh_helpers_front: chunk_helper_front_buffer,
            mesh_helpers_back: chunk_helper_back_buffer,
            mesh_helpers_bg: mesh_helper_bg,
            to_mesh_greedy_pl,
            mesh,
            chunks_requests_buffer: chunk_requests_buffer,
            chunks_requests_staging_buffer: chunk_requests_staging_buffer,
        }
    }
}
