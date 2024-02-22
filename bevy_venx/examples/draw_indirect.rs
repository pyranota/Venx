use std::{f32::consts::PI, slice::Chunks};

use bevy::{
    log,
    math::vec3,
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{mesh::GpuMesh, render_resource::PrimitiveTopology, renderer::RenderDevice},
    tasks::block_on,
};
use bevy_panorbit_camera::PanOrbitCamera;

use venx::{
    plat::{
        interfaces::{layer::LayerInterface, load::LoadInterface},
        VenxPlat,
    },
    BindGroupBuilder, Chunk, ComputeServer,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_panorbit_camera::PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.52, 0.80, 0.92)))
        .run();
}
fn setup(
    mut cmd: Commands,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    device: Res<RenderDevice>,
) {
    // let plat = VenxPlat::load("mca_mid").unwrap_or_else(|e| {
    //     warn!("Plat wasnt found on device, creating new and saving ({e})");
    //     // Convert from minecraft map
    //     let plat = VenxPlat::load_mca("./assets/mca/4/", (0..1, 0..1)).unwrap();
    //     plat.save("mca_mid").unwrap();
    //     plat
    // });

    // let mut plat = VenxPlat::new(6, 5, 5);

    // plat.set_voxel(0, (0, 0, 0).into(), 4);
    // plat.set_voxel(0, (0, 1, 0).into(), 9);

    // plat.set_voxel(0, (0, 5, 0).into(), 9);
    // info!("Transfer to gpu");
    // let plat = block_on(plat.transfer_to_gpu());

    // info!("Loading chunks");

    // plat.
    let buffer = block_on(async {
        let mut cs = ComputeServer::new().await;

        let buffer = cs.new_buffer(&[0]);

        let mut cs2 = ComputeServer::new().await;

        let bg = BindGroupBuilder::new()
            .insert(0, false, buffer.as_entire_binding())
            .build(&cs2);

        buffer
    });

    let mesh = GpuMesh {
        vertex_buffer: todo!(),
        vertex_count: todo!(),
        morph_targets: todo!(),
        buffer_info: todo!(),
        primitive_topology: todo!(),
        layout: todo!(),
    };

    let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // bevy_mesh.

    // bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.0.clone());
    // bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh.1.clone());
    // bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.2.clone());

    // cmd.spawn(PbrBundle {
    //     mesh: bevy_meshes.add(mesh),
    //     material: materials.add(StandardMaterial {
    //         reflectance: 0.1,
    //         base_color: Color::rgb(1., 1., 1.),
    //         // alpha_mode: AlphaMode::Blend,
    //         ..default()
    //     }),
    //     ..default()
    // })
    // //.insert(Wireframe)
    // ;

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.53,
    });
    // // light
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 4000.0,
    //         shadows_enabled: false,
    //         range: 400.,
    //         radius: 200.,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 15.0, 0.0),
    //     ..default()
    // });
    // directional 'sun' light
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-10.0, 300.0, -10.0),
            rotation: Quat::from_rotation_x(-PI / 3.87),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });

    // camera
    cmd.spawn((
        Camera3dBundle {
            camera: Camera {
                // hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(28.0, 50., 28.0).looking_at(vec3(0., 0., 0.), Vec3::Y),
            ..default()
        },
        // ScreenSpaceAmbientOcclusionBundle::default(),
        // TemporalAntiAliasBundle::default(),
        // FogSettings {
        //     color: Color::rgb(0.52, 0.80, 0.92),
        //     falloff: FogFalloff::Linear {
        //         start: 200.0,
        //         end: 5000.0,
        //     },
        //     ..Default::default()
        // },
        PanOrbitCamera::default(),
    ));
}
