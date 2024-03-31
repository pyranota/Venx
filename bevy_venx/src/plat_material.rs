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
/// Amount of faces (vertices * 6) per bucket
const BUCKET_SIZE: u32 = 512;

/// Amount of buckets in entire vertex pool
const VP_SIZE: u32 = 1024;

/// Amount of draw calls.
/// Also amount of possible bucket amount at the time
const DRAW_CALLS: u32 = VP_SIZE / BUCKET_SIZE;

pub(super) fn setup_voxel_pool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    device: Res<RenderDevice>,
) {
    let contents: Vec<u8> = (0..DRAW_CALLS)
        .map(|i| -> Vec<u8> {
            bytemuck::cast_slice(
                &DrawIndirectPacked {
                    vertex_count: BUCKET_SIZE * 6,
                    instance_count: 1,
                    base_vertex: i * BUCKET_SIZE * 6,
                    base_instance: 0,
                }
                .to_arr(),
            )
            .to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
        .concat();

    let indirect_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Venx plat indirect buffer"),
        contents: &contents,
        usage: BufferUsages::VERTEX | BufferUsages::INDIRECT | BufferUsages::COPY_DST,
    });

    // Zero'ing mesh.
    let mesh = Box::new(vec![Vec3::ZERO; (BUCKET_SIZE * 6 * VP_SIZE) as usize]);

    // Feeding to bevy
    let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, *mesh.clone());
    bevy_mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[0., 0., 0., 0.]; mesh.clone().len()],
    );
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; mesh.len()]);
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, *mesh);

    commands.spawn((
        meshes.add(bevy_mesh),
        SpatialBundle::INHERITED_IDENTITY,
        PlatMaterialData {
            indirect_buffer,
            draw_calls_count: DRAW_CALLS,
        },
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
        // VertexPool{

        // },
    ));
}

#[derive(Component)]
struct PlatMaterialData {
    indirect_buffer: Buffer,
    draw_calls_count: u32,
}

impl ExtractComponent for PlatMaterialData {
    type Query = &'static PlatMaterialData;
    type Filter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(PlatMaterialData {
            indirect_buffer: item.indirect_buffer.clone(),
            draw_calls_count: item.draw_calls_count.clone(),
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
            let key = view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
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
    draw_calls_count: u32,
}

fn prepare_instance_buffers(mut commands: Commands, query: Query<(Entity, &PlatMaterialData)>) {
    for (entity, instance_data) in &query {
        commands.entity(entity).insert(InstanceBuffer {
            indirect_buffer: instance_data.indirect_buffer.clone(),
            draw_calls_count: instance_data.draw_calls_count,
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
        item: &P,
        _view: (),
        instance_buffer: &'w InstanceBuffer,
        (meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        let gpu_mesh = match meshes.into_inner().get(mesh_instance.mesh_asset_id) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));

        pass.multi_draw_indirect(
            &instance_buffer.indirect_buffer,
            0,
            instance_buffer.draw_calls_count as u32,
        );
        RenderCommandResult::Success
    }
}
