use std::f32::consts::PI;

use bevy::{
    math::vec3, pbr::wireframe::Wireframe, prelude::*, render::render_resource::PrimitiveTopology,
};
use bevy_panorbit_camera::PanOrbitCamera;
use venx::plat::VenxPlat;

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
) {
    // Its small-sized plat, its slow to convert it from mca each run, it will be saved
    // let plat = VenxPlat::load("ALOD").unwrap_or_else(|e| {
    //     warn!("Plat wasnt found on device, creating new and saving ({e})");
    //     // Convert from minecraft map
    //     let plat = VenxPlat::load_mca("./assets/mca/1/", (0..1, 0..1), false, 0, true).unwrap();
    //     plat.save("ALOD").unwrap();
    //     plat
    // });

    let plat = VenxPlat::load("ALOD").unwrap();
    for mesh in plat.static_mesh(0..16, 3..7, 0..16, Some(1)) {
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
        .insert(Wireframe);
    }

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
            transform: Transform::from_xyz(512.0, 200., 512.0)
                .looking_at(vec3(512., 0., 512.), Vec3::Y),
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
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(452., 190., 452.),
            ..Default::default()
        },
    ));
}
