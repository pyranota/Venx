//! A simple 3D scene with light shining over a cube sitting on a plane.
// pub mod fps;
pub mod plat;

use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin},
    asset::Assets,
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::system::{Commands, ResMut},
    math::vec3,
    pbr::{wireframe::WireframePlugin, PbrBundle, StandardMaterial},
    prelude::*,
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
};
use bevy_panorbit_camera::PanOrbitCamera;

pub struct Venx;

impl Plugin for Venx {
    fn build(&self, app: &mut App) {
        dbg!("Add Venx");
        app.add_plugins(
            // bevy_panorbit_camera::PanOrbitCameraPlugin,
            // WireframePlugin,
            (
                FrameTimeDiagnosticsPlugin,
                bevy_panorbit_camera::PanOrbitCameraPlugin,
                WireframePlugin,
            ), // TemporalAntiAliasPlugin,
               // MaterialPlugin::<CustomMaterial>::default(),
        )
        .add_systems(Startup, setup)
        // .add_systems((fps_text_update_system, fps_counter_showhide))
        .insert_resource(ClearColor(Color::rgb(0.52, 0.80, 0.92)));
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    dbg!("Setup Venx");
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 10.3).into()),
        ..default()
    });
    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.13,
    });
    // // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            range: 400.,
            radius: 200.,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
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

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(-100.0, 300.0, -10.0),
    //         rotation: Quat::from_rotation_x(-PI / 3.87),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     // cascade_shadow_config: CascadeShadowConfigBuilder {
    //     //     first_cascade_far_bound: 4.0,
    //     //     maximum_distance: 10.0,
    //     //     ..default()
    //     // }
    //     // .into(),
    //     ..default()
    // });
    dbg!("Spawn Camera");
    // camera
    commands.spawn((
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

    dbg!("End of setup");
}
