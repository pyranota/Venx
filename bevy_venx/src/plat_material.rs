use std::any::Any;

use bevy::math::vec3;
use bevy::render::renderer::RenderQueue;
use bevy::tasks::block_on;
use bevy::{core::Pod, prelude::*, render::render_resource::PrimitiveTopology};

use bevy::render::render_resource::BufferInitDescriptor;

use bytemuck::Zeroable;

use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    },
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        view::{ExtractedView, NoFrustumCulling},
        Render, RenderApp, RenderSet,
    },
};
use venx::plat::interfaces::layer::LayerInterface;
use venx::plat::loader::external_buffer::EXTERNAL_BUFFER_RESOLUTION;
use venx::plat::loader::vertex_pool::VertexPool;
use venx::plat::VenxPlat;

use crate::plat::BevyPlat;
use crate::vertex_pool::PoolBuffer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DrawIndirectPacked {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

impl DrawIndirectPacked {
    pub fn size() -> u64 {
        16
    }
    pub fn to_arr(self) -> [u32; 4] {
        [
            self.vertex_count,
            self.instance_count,
            self.base_vertex,
            self.base_instance,
        ]
    }
}
// TODO: Remove
/// Amount of faces (vertices * 6) per bucket
pub const BUCKET_SIZE: u32 = 512;

// TODO: Remove
/// Amount of buckets in entire vertex pool
const VP_SIZE: u32 = 1024 * 6;

// TODO: Remove
/// Amount of draw calls.
/// Also amount of possible bucket amount at the time
const DRAW_CALLS: u32 = VP_SIZE / BUCKET_SIZE;

pub(super) fn setup_voxel_pool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    // Indirect buffer accessible with PlatMaterial and VertexPoolComponent
    let indirect_pool_buffer;
    let indirect_buffer;
    {
        // Each Bucket uses single draw call
        let contents: Vec<u8> = (0..DRAW_CALLS)
            .map(|i| -> Vec<u8> {
                bytemuck::cast_slice(
                    &DrawIndirectPacked {
                        vertex_count: BUCKET_SIZE * 6,
                        instance_count: 1,
                        // Basically offset of bucket
                        base_vertex: i * BUCKET_SIZE * 6,
                        base_instance: 0,
                    }
                    .to_arr(),
                )
                .to_vec()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat();

        indirect_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Venx plat indirect buffer"),
            contents: &contents,
            usage: BufferUsages::VERTEX | BufferUsages::INDIRECT | BufferUsages::COPY_DST,
        });

        let indirect_staging_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Venx plat staging indirect buffer"),
            usage: BufferUsages::VERTEX | BufferUsages::INDIRECT | BufferUsages::COPY_DST,
            size: EXTERNAL_BUFFER_RESOLUTION,
            mapped_at_creation: false,
        });

        indirect_pool_buffer = PoolBuffer {
            device: device.clone(),
            queue: queue.clone(),
            buffer: indirect_buffer.clone(),
            staging_buffer: indirect_staging_buffer,
        };
    }
    // Create primary vertex buffer with all vertices data.
    // This buffer is known and can be mapped at any time
    // Accessible with [VertexPoolComponent]
    let vertex_buffer;
    let vertex_pool_buffer;
    {
        // Zero'ing mesh.
        let mut mesh = Box::new(vec![Vec3::ZERO; (BUCKET_SIZE * 6 * VP_SIZE) as usize]);

        // for (i, vertices) in mesh.as_mut_slice().chunks_mut(3).enumerate() {
        //     let offset = vec3(i as f32, i as f32, i as f32);
        //     vertices[0] = vec3(0., 10., 0.) + offset;
        //     vertices[1] = vec3(10., 0., 0.) + offset;
        //     vertices[2] = vec3(0., 0., 10.) + offset;
        // }

        // Feeding to bevy
        let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, *mesh.clone());
        bevy_mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[0., 0., 1., 0.5]; mesh.clone().len()],
        );
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; mesh.len()]);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, *mesh);

        let vertex_buffer_data = bevy_mesh.get_vertex_buffer_data();
        vertex_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            label: Some("Mesh Vertex Buffer"),
            contents: &vertex_buffer_data,
        });
        let vertex_staging_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Venx plat staging vertex buffer"),
            usage: BufferUsages::COPY_SRC | BufferUsages::MAP_WRITE,
            contents: bytemuck::cast_slice(&vec![0; EXTERNAL_BUFFER_RESOLUTION as usize]),
        });
        vertex_pool_buffer = PoolBuffer {
            device: device.clone(),
            queue: queue.clone(),
            buffer: vertex_buffer.clone(),
            staging_buffer: vertex_staging_buffer,
        };
    }
    // We need this mesh to provide right mesh layout and to tweak position of platform
    // TODO: Unhardcode mesh layout
    let mut sm_bevy_mesh;
    {
        let small_mesh = Box::new(vec![Vec3::ZERO; 3]);
        sm_bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        sm_bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, *small_mesh.clone());
        sm_bevy_mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[0., 0., 1., 0.5]; small_mesh.clone().len()],
        );
        sm_bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; small_mesh.len()]);
        sm_bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, *small_mesh);
    }

    let vertex_pool = VertexPool::new(
        // TODO: Unhardcode
        256,
        6500,
        vec![500, 1000, 5000],
        Box::new(indirect_pool_buffer),
        Box::new(vertex_pool_buffer),
    );
    // TODO: Unhardcode
    // let mut plat = VenxPlat::load("sm", vertex_pool).unwrap();

    let mut plat = VenxPlat::new(8, 5, 6, vertex_pool);
    block_on(async {
        plat.set_voxel(0, (5, 5, 5).into(), 2).await;
        plat.set_voxel(0, (9, 5, 5).into(), 2).await;
        plat.set_voxel(0, (1, 5, 5).into(), 2).await;
        plat.set_voxel(0, (4, 5, 5).into(), 2).await;
    });

    dbg!(plat.get_voxel((5, 5, 5).into()));
    commands.spawn((
        meshes.add(sm_bevy_mesh.clone()),
        SpatialBundle::INHERITED_IDENTITY,
        PlatMaterialData {
            indirect_buffer: indirect_buffer.clone(),
            draw_calls_count: DRAW_CALLS,
            vertex_buffer,
        },
        NoFrustumCulling,
        BevyPlat(plat),
    ));
}

#[derive(Component)]
pub struct VertexPoolComponent {
    pub vertex_pool: VertexPool,
}

#[derive(Component)]
pub struct PlatMaterialData {
    pub indirect_buffer: Buffer,
    pub draw_calls_count: u32,
    pub vertex_buffer: Buffer,
}

impl ExtractComponent for PlatMaterialData {
    type Query = &'static PlatMaterialData;
    type Filter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(PlatMaterialData {
            indirect_buffer: item.indirect_buffer.clone(),
            draw_calls_count: item.draw_calls_count.clone(),
            vertex_buffer: item.vertex_buffer.clone(),
        })
    }
}

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<PlatMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<CustomPipeline>();
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    material_meshes: Query<Entity, With<PlatMaterialData>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut transparent_phase) in &mut views {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for entity in &material_meshes {
            let Some(mesh_instance) = render_mesh_instances.get(&entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key = view_key
                | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: rangefinder
                    .distance_translation(&mesh_instance.transforms.transform.translation),
                batch_range: 0..1,
                dynamic_offset: None,
            });
        }
    }
}

#[derive(Component)]
pub struct InstanceBuffer {
    indirect_buffer: Buffer,
    vertex_buffer: Buffer,
    draw_calls_count: u32,
}

// TODO: Remove and just use [PlatMaterialData]
fn prepare_instance_buffers(mut commands: Commands, query: Query<(Entity, &PlatMaterialData)>) {
    for (entity, instance_data) in &query {
        commands.entity(entity).insert(InstanceBuffer {
            indirect_buffer: instance_data.indirect_buffer.clone(),
            draw_calls_count: instance_data.draw_calls_count,
            vertex_buffer: instance_data.vertex_buffer.clone(),
        });
    }
}

#[derive(Resource)]
pub struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server.load("shaders/plat_material.wgsl");

        let mesh_pipeline = world.resource::<MeshPipeline>();

        CustomPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedMeshPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        // meshes typically live in bind group 2. because we are using bindgroup 1
        // we need to add MESH_BINDGROUP_1 shader def so that the bindings are correctly
        // linked in the shader
        descriptor
            .vertex
            .shader_defs
            .push("MESH_BINDGROUP_1".into());

        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMeshInstanced,
);

pub struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<RenderMeshInstances>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<InstanceBuffer>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        instance_buffer: &'w InstanceBuffer,
        (_meshes, _render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let vertex_buffer = &instance_buffer.vertex_buffer;
        // let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else {
        //     return RenderCommandResult::Failure;
        // };

        // let gpu_mesh = match meshes.into_inner().get(mesh_instance.mesh_asset_id) {
        //     Some(gpu_mesh) => gpu_mesh,
        //     None => return RenderCommandResult::Failure,
        // };

        // let vertex_buffer = &gpu_mesh.vertex_buffer;

        // dbg!(gpu_mesh.vertex_buffer.usage());
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.multi_draw_indirect(
            &instance_buffer.indirect_buffer,
            0,
            instance_buffer.draw_calls_count as u32,
        );
        RenderCommandResult::Success
    }
}
