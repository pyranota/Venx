use std::{thread, time::Duration};

use bevy::{
    core::Pod,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    pbr::{wireframe::Wireframe, CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::{render_resource::PrimitiveTopology, renderer::RenderQueue},
    tasks::{block_on, AsyncComputeTaskPool, ComputeTaskPool, Task, TaskPool},
    utils::tracing::instrument::WithSubscriber,
    window::PresentMode,
};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_venx::{
    fps_counter::{fps_counter_showhide, fps_setup_counter, fps_text_update_system},
    plat_material::DrawIndirectPacked,
};
use bytemuck::Zeroable;
use rand::Rng;

use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::{GpuBufferInfo, MeshVertexBufferLayout},
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

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Disable VSync for more accurate result
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }),
        CustomMaterialPlugin, // Adds frame time diagnostics
        FrameTimeDiagnosticsPlugin,
        // Adds a system that prints diagnostics to the console
        //LogDiagnosticsPlugin::default(),
    ))
    .add_systems(Startup, (setup, fps_setup_counter))
    .add_systems(Update, (fps_counter_showhide, fps_text_update_system, poll))
    .add_systems(Update, blink);

    app.run();
}

fn poll(device: Res<RenderDevice>, queue: Res<RenderQueue>) {
    let encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Encoder polling for map_async to work"),
    });

    queue.submit(Some(encoder.finish()));
}

#[derive(Component)]
pub struct UpdateBufferTask {
    pub task: Option<Task<()>>,
}

fn blink(
    mut q: Query<(&InstanceMaterialData, &mut UpdateBufferTask)>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    for (data, mut task) in &mut q {
        if task.task.is_none() {
            let staging_buffer = data.indirect_buffer_staging.clone();
            let indirect_buffer = data.indirect_buffer.clone();

            let mut rng = rand::thread_rng();
            let offset = DrawIndirectPacked::size() * rng.gen_range(1..(140_000 - 8));
            let instance_count = if rng.gen_range(0..30) > 2 { 0 } else { 1 };
            let (cloned_device, cloned_queue) = (device.clone(), queue.clone());
            let fut = async move {
                // //     let mut encoder = self
                // //     .device
                // //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                // // {
                // //     closure(&mut encoder);
                // // }
                // // // Submits command encoder for processing
                // // self.queue.submit(Some(encoder.finish()));

                // let clone = staging_buffer.clone();

                let bucket_size = 36;

                let size = DrawIndirectPacked::size() * 7;

                let buffer_slice = staging_buffer.slice(offset..(size + offset));

                // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
                let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
                buffer_slice.map_async(MapMode::Write, move |v| sender.send(v).unwrap());

                // Awaits until `buffer_future` can be read from
                if let Some(Ok(())) = receiver.receive().await {
                    let mut data = buffer_slice.get_mapped_range_mut();
                    let draw_indirect_slice: &mut [DrawIndirectPacked] =
                        bytemuck::cast_slice_mut(data.as_mut());
                    // Hopefully should hide first cube for draw_indirect_el in draw_indirect_slice { draw_indirect_el.instance_count = instance_count; }
                }

                staging_buffer.unmap();

                let mut encoder = cloned_device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some(
                        "Encoder for copying data from indirect staging buffer to storage buffer",
                    ),
                });

                encoder.copy_buffer_to_buffer(
                    &staging_buffer,
                    offset,
                    &indirect_buffer,
                    offset,
                    size,
                );

                cloned_queue.submit(Some(encoder.finish()));
            };
            let thread_pool = bevy::tasks::AsyncComputeTaskPool::get();

            let new_task = thread_pool.spawn(async move {});
            task.task = Some(new_task);
        } else {
            if let Some(task_progress) = &task.task {
                if task_progress.is_finished() {
                    task.task = None;
                }
            }
        }
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, device: Res<RenderDevice>) {
    pub const FULL_CUBE: [Vec3; 36] = [
        // front face
        Vec3::new(-0., -0., 1.0),
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., -0., 1.0),
        // back face
        Vec3::new(1.0, -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, -0., -0.),
        // top face
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, 1.0),
        // bottom face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, -0., 1.0),
        // right face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -0., 1.0),
        // left face
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., -0., -0.),
    ];

    let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut mesh = vec![];

    // Draw x00_000 cubes at 60fps each in its own draw call!
    let _: Vec<()> = (0..140_000)
        .map(|x| {
            for vertex in &FULL_CUBE {
                let bound = 300.;
                let y = x as f32 % bound * 1.5 - 200.;
                let x = x as f32 / bound * 1.5 - 350.;
                mesh.push(*vertex + vec3(x, y, 0.));
            }
        })
        .collect();

    dbg!(mesh.len());

    // let mesh = [FULL_CUBE; 500].concat();

    let bucket_size = 36;

    let draw_calls_count = mesh.len() as u32 / bucket_size;

    dbg!(draw_calls_count);
    let _to_dbg: Vec<DrawIndirect> = (0..draw_calls_count)
        .map(|i| DrawIndirect {
            vertex_count: bucket_size,
            instance_count: 1,
            base_vertex: i as u32 * bucket_size,
            base_instance: 0,
        })
        .collect();

    let contents: Vec<u8> = (0..draw_calls_count)
        .map(|i| {
            DrawIndirect {
                vertex_count: bucket_size,
                instance_count: 1,
                base_vertex: i as u32 * bucket_size,
                base_instance: 0,
            }
            .as_bytes()
            .to_owned()
        })
        .collect::<Vec<Vec<u8>>>()
        .concat();

    let indirect_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Venx plat indirect buffer"),
        contents: &contents,
        usage: BufferUsages::STORAGE | BufferUsages::INDIRECT | BufferUsages::COPY_DST,
    });

    let indirect_buffer_staging = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Venx plat indirect buffer"),
        usage: //BufferUsages::STORAGE
          //  | BufferUsages::INDIRECT
             BufferUsages::MAP_WRITE
            | BufferUsages::COPY_SRC,
            contents: &contents,

    });

    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.clone());
    bevy_mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1., 1., 1., 1.]; mesh.clone().len()],
    );
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; mesh.len()]);
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh);

    commands.spawn((
        meshes.add(bevy_mesh),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData {
            indirect_buffer: indirect_buffer,
            draw_calls_count: draw_calls_count,
            indirect_buffer_staging: indirect_buffer_staging,
        },
        UpdateBufferTask { task: None },
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 650.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component, ExtractComponent, Clone)]
struct InstanceMaterialData {
    indirect_buffer: Buffer,
    indirect_buffer_staging: Buffer,
    draw_calls_count: u32,
}

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
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
    material_meshes: Query<Entity, With<InstanceMaterialData>>,
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
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        commands.entity(entity).insert(InstanceBuffer {
            indirect_buffer: instance_data.indirect_buffer.clone(),
            draw_calls_count: instance_data.draw_calls_count,
            length: 0,
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
        // dbg!(&descriptor.vertex.buffers);
        // descriptor.vertex.buffers.push(VertexBufferLayout {
        //     array_stride: std::mem::size_of::<InstanceData>() as u64,
        //     step_mode: VertexStepMode::Instance,
        //     attributes: vec![
        //         VertexAttribute {
        //             format: VertexFormat::Float32x4,
        //             offset: 0,
        //             shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
        //         },
        //         VertexAttribute {
        //             format: VertexFormat::Float32x4,
        //             offset: VertexFormat::Float32x4.size(),
        //             shader_location: 4,
        //         },
        //     ],
        // });
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

        //dbg!(instance_buffer.vertex_buffer.size());

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        // pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        let offset = 0;
        pass.multi_draw_indirect(
            &instance_buffer.indirect_buffer,
            offset,
            instance_buffer.draw_calls_count as u32 - offset as u32,
        );

        //pass.draw(0..6, 0..1);

        // match &gpu_mesh.buffer_info {
        //     GpuBufferInfo::Indexed {
        //         buffer,
        //         index_format,
        //         count,
        //     } => {
        //         dbg!(count);
        //         pass.set_index_buffer(buffer.slice(..), 0, *index_format);
        //         pass.draw_indexed(0..*count, 0, 0..1);
        //     }
        //     GpuBufferInfo::NonIndexed => {
        //         pass.draw(0..gpu_mesh.vertex_count, 0..instance_buffer.length as u32);
        //     }
        // }
        RenderCommandResult::Success
    }
}
// fn setup_my(
//     mut cmd: Commands,
//     mut bevy_meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     device: Res<RenderDevice>,
// ) {
//     dbg!(device.wgpu_device());

//     let vertex_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
//         label: Some("Venx plat vertex data"),
//         contents: &[],
//         usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
//     });

//     let indirect_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
//         label: Some("Venx plat indirect buffer"),
//         contents: &[],
//         usage: BufferUsages::STORAGE | BufferUsages::INDIRECT,
//     });

//     return;
//     // Its small-sized plat, its slow to convert it from mca each run, it will be saved
//     let plat = VenxPlat::load("sm").unwrap_or_else(|e| {
//         warn!("Plat wasnt found on device, creating new and saving ({e})");
//         // Convert from minecraft map
//         let plat = VenxPlat::load_mca_untyped("./assets/mca/4/", (0..4, 0..4)).unwrap();
//         plat.save("sm").unwrap();
//         plat
//     });

//     const BUCKET_SIZE: usize = 1024;

//     // let plat = VenxPlat::load_mca("./assets/mca/1/", (0..5, 0..5), true, 100, true).unwrap();
//     for mesh in plat.static_mesh(0..16, 3..8, 0..16, Some(0)) {
//         let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);

//         bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.0.clone());
//         bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh.1.clone());
//         bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.2.clone());

//         cmd.spawn(PbrBundle {
//             mesh: bevy_meshes.add(bevy_mesh),
//             material: materials.add(StandardMaterial {
//                 reflectance: 0.1,
//                 base_color: Color::rgb(1., 1., 1.),
//                 // alpha_mode: AlphaMode::Blend,
//                 ..default()
//             }),
//             ..default()
//         })
//         .insert(Wireframe);
//     }

//     // ambient light
//     cmd.insert_resource(AmbientLight {
//         color: Color::WHITE,
//         brightness: 0.15,
//     });
//     // // light
//     // cmd.spawn(PointLightBundle {
//     //     point_light: PointLight {
//     //         intensity: 5000000.0,
//     //         shadows_enabled: true,
//     //         range: 4000.,
//     //         radius: 2000.,
//     //         color: Color::YELLOW,
//     //         ..Default::default()
//     //     },
//     //     transform: Transform {
//     //         translation: Vec3::new(-10.0, 500.0, -10.0),
//     //         rotation: Quat::from_rotation_x(-PI / 3.87),
//     //         ..default()
//     //     },
//     //     ..default()
//     // });

//     // // // light
//     // cmd.spawn(PointLightBundle {
//     //     point_light: PointLight {
//     //         intensity: 5000000.0,
//     //         shadows_enabled: true,
//     //         range: 4000.,
//     //         radius: 2000.,
//     //         color: Color::WHITE,
//     //         ..default()
//     //     },
//     //     transform: Transform {
//     //         translation: Vec3::new(300.0, 500.0, -10.0),
//     //         rotation: Quat::from_rotation_x(-PI / 3.87),
//     //         ..default()
//     //     },
//     //     ..default()
//     // });
//     // directional 'sun' light
//     // cmd.spawn(DirectionalLightBundle {
//     //     directional_light: DirectionalLight {
//     //         shadows_enabled: true,
//     //         illuminance: 35_000.,
//     //         ..Default::default()
//     //     },
//     //     transform: Transform {
//     //         translation: Vec3::new(1000.0, -1000.0, 1000.0),
//     //         // rotation: Quat::from_rotation_x(-PI / 3.87),
//     //         ..default()
//     //     },
//     //     // The default cascade config is designed to handle large scenes.
//     //     // As this example has a much smaller world, we can tighten the shadow
//     //     // bounds for better visual quality.
//     //     cascade_shadow_config: CascadeShadowConfigBuilder {
//     //         first_cascade_far_bound: 200.0,
//     //         maximum_distance: 5000.0,
//     //         ..default()
//     //     }
//     //     .into(),
//     //     ..default()
//     // });
//     let cascade_shadow_config = CascadeShadowConfigBuilder {
//         first_cascade_far_bound: 0.3,
//         maximum_distance: 3.0,
//         ..default()
//     }
//     .build();

//     // Sun
//     cmd.spawn(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             color: Color::rgb(0.98, 0.95, 0.82),
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(300.0, 300.0, 300.0)
//             .looking_at(Vec3::new(-0.15, -0.05, -0.25), Vec3::Y),
//         cascade_shadow_config,
//         ..default()
//     });
//     // // Sky
//     // cmd.spawn((
//     //     PbrBundle {
//     //         mesh: bevy_meshes.add(Mesh::from(shape::Box::default())),
//     //         material: materials.add(StandardMaterial {
//     //             base_color: Color::hex("888888").unwrap(),
//     //             unlit: true,
//     //             cull_mode: None,
//     //             ..default()
//     //         }),
//     //         transform: Transform::from_scale(Vec3::splat(1900.0)),
//     //         ..default()
//     //     },
//     //     NotShadowCaster,
//     // ));

//     // camera
//     cmd.spawn((
//         Camera3dBundle {
//             camera: Camera {
//                 hdr: true,
//                 ..default()
//             },
//             transform: Transform::from_xyz(28.0, 200., 28.0)
//                 .looking_at(Vec3::new(-0.15, -0.05, -0.25), Vec3::Y),
//             ..default()
//         },
//         // ScreenSpaceAmbientOcclusionBundle::default(),
//         // TemporalAntiAliasBundle::default(),
//         // FogSettings {
//         //     color: Color::rgb(0.52, 0.80, 0.92),
//         //     // falloff: FogFalloff::Atmospheric { extinction: (), inscattering: () } {
//         //     //     start: 200.0,
//         //     //     end: 5000.0,
//         //     // },
//         //     falloff: FogFalloff::from_visibility(3050.0),
//         //     ..Default::default()
//         // },
//         FogSettings {
//             color: Color::rgba(0.35, 0.48, 0.66, 1.0),
//             directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
//             directional_light_exponent: 30.0,
//             falloff: FogFalloff::from_visibility_colors(
//                 1000.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
//                 Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
//                 Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
//             ),
//         },
//         PanOrbitCamera {
//             // Set focal point (what the camera should look at)
//             focus: Vec3::new(280.0, 228., 280.0),
//             ..Default::default()
//         },
//     ));
// }
