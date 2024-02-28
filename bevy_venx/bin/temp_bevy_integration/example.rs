use bevy::render::render_resource::PrimitiveTopology;

use bevy::prelude::*;

use main::Venx;

mod main;

use venx::plat::VenxPlat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Venx))
        .add_systems(Startup, setup)
        .run();
}
fn setup(
    mut cmd: Commands,

    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Convert from minecraft map
    let plat = VenxPlat::load_mca("./assets/mca/1/", (0..1, 0..1), false, 0, true).unwrap();

    for mesh in plat.static_mesh(0..16, 0..6, 0..16, None) {
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
}
