use std::{f32::consts::PI, slice::Chunks};

use bevy::{
    math::vec3, pbr::wireframe::Wireframe, prelude::*, render::render_resource::PrimitiveTopology,
};
use bevy_panorbit_camera::PanOrbitCamera;
use pollster::block_on;
use venx::{
    plat::{
        interfaces::{layer::LayerInterface, load::LoadInterface},
        VenxPlat,
    },
    Chunk,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_panorbit_camera::PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.52, 0.80, 0.92)))
        .run();
}
fn setup(
    mut cmd: Commands,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Its small-sized plat, its slow to convert it from mca each run, it will be saved
    let plat = VenxPlat::load("mca_small").unwrap_or_else(|e| {
        warn!("Plat wasnt found on device, creating new and saving ({e})");
        // Convert from minecraft map
        let plat = VenxPlat::load_mca("./assets/mca/1/", (0..1, 0..1)).unwrap();
        plat.save("mca_small").unwrap();
        plat
    });

    // let plat_2 = VenxPlat::load("mca_small").unwrap_or_else(|e| {
    //     warn!("Plat wasnt found on device, creating new and saving ({e})");
    //     // Convert from minecraft map
    //     let plat = VenxPlat::load_mca("./assets/mca/1/", (0..1, 0..1)).unwrap();
    //     plat.save("mca_small").unwrap();
    //     plat
    // });

    // let mut plat = VenxPlat::new(6, 5, 5);

    // plat.set_voxel(0, (0, 0, 0).into(), 4);
    // plat.set_voxel(0, (0, 1, 0).into(), 9);

    // plat.set_voxel(0, (0, 5, 0).into(), 9);
    info!("Transfer to gpu");
    let plat = block_on(plat.transfer_to_gpu());

    info!("Loading chunks");

    let mut blank_chunk = Box::new(vec![]);

    for x in 0..16 {
        for y in 3..7 {
            for z in 0..16 {
                blank_chunk.push(Chunk::new((x, y, z), 0, 5))
            }
        }
    }

    let chunks = plat.load_chunks(blank_chunk);

    info!("Transfer from gpu");
    let plat = block_on(plat.transfer_from_gpu());

    // assert_eq!(
    //     plat.get_normal_unchecked().borrow_raw_plat().depth,
    //     plat_2.get_normal_unchecked().borrow_raw_plat().depth
    // );
    info!("Computing meshes and spawning 3d models");
    for chunk in chunks.iter() {
        let mesh = plat.compute_mesh_from_chunk(&chunk);
        //assert!(chunk.get((0, 2, 0).into()).is_some());

        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut colors: Vec<[f32; 4]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];

        for (position, color, normal) in mesh.iter() {
            if color.to_array() == glam::f32::Vec4::ZERO.to_array() {
                break;
            }

            vertices.push(position.to_array());
            colors.push(color.to_array());
            normals.push(normal.to_array());
        }
        let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

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
        .insert(Wireframe);
    }

    info!("Inserting the rest");
    // assert!(chunk.get((0, 0, 0).into()).is_some());
    // assert!(chunk.get((0, 1, 0).into()).is_some());

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.13,
    });
    // // light
    cmd.spawn(PointLightBundle {
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
    cmd.spawn(DirectionalLightBundle {
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
