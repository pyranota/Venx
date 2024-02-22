use std::f32::consts::PI;

use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    math::{uvec3, vec3},
    pbr::{
        CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster,
        ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
    },
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_venx::plat::BevyPlat;
use bevy_water::{
    material::{StandardWaterMaterial, WaterMaterial},
    WaterPlugin, WaterSettings,
};
use venx::plat::{interfaces::layer::LayerInterface, VenxPlat};

#[derive(Component)]
struct Character;

#[derive(Component)]
struct MidAir;

#[derive(Component)]
struct Velocity(pub Vec3);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
            WaterPlugin,
            TemporalAntiAliasPlugin,
        ))
        .insert_resource(WaterSettings {
            //  edge_scale: 5.
            spawn_tiles: None,
            height: 121.,
            edge_color: Color::hex("b0dbd8").unwrap(),
            edge_scale: 3.,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_velocity,
                apply_velocity,
                handle_collisions,
                move_character,
            ),
        )
        .insert_resource(ClearColor(Color::hex("87ceeb").unwrap()))
        .insert_resource(DirectionalLightShadowMap { size: 1512 })
        .run();
}

fn move_character(
    keys: Res<Input<KeyCode>>,
    mut q: Query<(Entity, &mut Transform, &mut Velocity), Without<MidAir>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, mut t, mut v) in &mut q {
        if keys.just_pressed(KeyCode::Space) {
            // Space was pressed
            dbg!("Jump");
            commands.entity(e).insert(MidAir);
            v.0.y += time.delta_seconds() * 30.;
            t.translation.y += 4.0;
        }
    }
}

fn handle_collisions(
    mut q: Query<(Entity, &mut Transform, &mut Velocity), With<MidAir>>,
    time: Res<Time>,
    mut pq: Query<&BevyPlat>,
    mut commands: Commands,
) {
    for (e, mut t, mut v) in &mut q {
        let rounded = t.translation.as_uvec3();
        // if v.0.y >= 0. {
        //     return;
        // }
        for plat in &pq {
            if plat.0.get_voxel(rounded.to_array().into()).is_some() {
                t.translation = Vec3::from_array(rounded.as_vec3().to_array()) + vec3(0., 0.5, 0.);
                commands.entity(e).remove::<MidAir>();
                v.0 = Vec3::ZERO;
            }
        }
    }
}
/// Add gravity
fn apply_velocity(
    mut q: Query<&mut Velocity, With<MidAir>>,
    mut q2: Query<&mut Velocity, Without<MidAir>>,
    time: Res<Time>,
) {
    for mut v in &mut q {
        v.0.y -= 1.0 * time.delta_seconds();
    }

    for mut v in &mut q2 {
        v.0.y = 0.;
    }
}

/// Project velocity into transform
fn handle_velocity(mut q: Query<(&mut Transform, &Velocity), With<Character>>, time: Res<Time>) {
    for (mut t, v) in &mut q {
        t.translation += v.0;
    }
}

fn setup(
    mut cmd: Commands,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<WaterSettings>,
    mut water_materials: ResMut<Assets<StandardWaterMaterial>>,
) {
    // Its mid-sized plat, its slow to convert it from mca each run, it will be saved
    let plat = VenxPlat::load("demo").unwrap_or_else(|e| {
        warn!("Plat wasnt found on device, creating new and saving ({e})");
        // Convert from minecraft map
        let plat = VenxPlat::load_mca("./assets/mca/49/", (0..8, 0..8)).unwrap();
        plat.save("demo").unwrap();
        plat
    });

    for mesh in plat.static_mesh(0..(16 * 2), 3..6, 0..(16 * 2), Some(1)) {
        let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);

        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.0.clone());
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh.1.clone());
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.2.clone());

        cmd.spawn(PbrBundle {
            mesh: bevy_meshes.add(bevy_mesh),
            material: materials.add(StandardMaterial {
                reflectance: 0.1,
                base_color: Color::rgb(1., 1., 1.),
                // alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        })
        //.insert(Wireframe)
        ;
    }

    cmd.spawn(BevyPlat(plat));

    // Mesh for water.
    let mesh: Handle<Mesh> = bevy_meshes.add(shape::Plane::from_size(400.).into());
    // Water material.
    let material = water_materials.add(StandardWaterMaterial {
        base: default(),
        extension: WaterMaterial {
            amplitude: settings.amplitude,
            coord_scale: Vec2::new(60224.0, 60224.0),
            ..default()
        },
    });

    let mut tr = Transform::from_scale(vec3(30., 1., 30.));

    tr.translation.y += 123.;

    cmd.spawn((
        Name::new(format!("Water world")),
        MaterialMeshBundle {
            mesh,
            material,
            transform: tr,

            ..default()
        },
        NotShadowCaster,
    ));

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.12,
    });
    // // // light
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 5000000.0,
    //         shadows_enabled: true,
    //         range: 4000.,
    //         radius: 2000.,
    //         color: Color::YELLOW,
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(-10.0, 500.0, -10.0),
    //         rotation: Quat::from_rotation_x(-PI / 3.87),
    //         ..default()
    //     },
    //     ..default()
    // });
    // // // light
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 5000000.0,
    //         shadows_enabled: true,
    //         range: 4000.,
    //         radius: 2000.,
    //         color: Color::YELLOW,
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(10.0, 500.0, -10.0),
    //         rotation: Quat::from_rotation_x(-PI / 3.87),
    //         ..default()
    //     },
    //     ..default()
    // });

    // // light
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.0,
            shadows_enabled: true,
            range: 4000.,
            radius: 2000.,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 1000.0, -10.0),
            rotation: Quat::from_rotation_x(-PI / 3.87),
            ..default()
        },
        ..default()
    });
    // // light
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.0,
            shadows_enabled: true,
            range: 4000.,
            radius: 2000.,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-10.0, 1000.0, -10.0),
            rotation: Quat::from_rotation_x(-PI / 3.87),
            ..default()
        },
        ..default()
    });
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 500.,
        maximum_distance: 3500.0,
        ..default()
    }
    .build();
    // // Sun
    // cmd.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         color: Color::rgb(0.98, 0.95, 0.82),
    //         illuminance: 23_000.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(-200.0, 300.0, -200.0)
    //         .looking_at(Vec3::new(0.15, 0.05, 0.45), Vec3::Y),
    //     cascade_shadow_config,
    //     ..default()
    // });
    // // directional 'sun' light
    // cmd.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(-10.0, 300.0, -10.0),
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
    // Character
    cmd.spawn(PbrBundle {
        mesh: bevy_meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(170.0, 150.5, 40.0),
        ..default()
    })
    .insert(Character)
    .insert(MidAir)
    .insert(Velocity(Vec3::ZERO))
    .with_children(|b| {
        b.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(28.0, 50., 28.0)
                    .looking_at(vec3(0., 0., 0.), Vec3::Y),
                ..default()
            },
            ScreenSpaceAmbientOcclusionBundle {
                settings: ScreenSpaceAmbientOcclusionSettings {
                    quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                },
                depth_prepass: bevy::core_pipeline::prepass::DepthPrepass,
                normal_prepass: bevy::core_pipeline::prepass::NormalPrepass,
            },
            // TemporalAntiAliasBundle::default(),
            FogSettings {
                color: Color::hex("87ceeb").unwrap(),
                falloff: FogFalloff::Linear {
                    start: 300.0,
                    end: 4000.0,
                },
                ..Default::default()
            },
            PanOrbitCamera::default(),
        ));
    });

    // camera
    //cmd
}
