use std::time::{Duration, Instant};

use bevy::pbr::wireframe::Wireframe;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::PresentMode;
use bevy::{log, prelude::*};
use glam::{uvec3, vec4};

use main::Venx;
use venx::plat::interfaces::layer::LayerInterface;
use venx::plat::interfaces::load::LoadInterface;
mod main;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use venx::plat::VenxPlat;

fn main() {
    dbg!("Start app");
    info!("Starting the program");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Venx)
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut cmd: Commands,

    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    dbg!("Start setup");
    info!("Starting the program");
    let mut meshes = [
        Mesh::new(PrimitiveTopology::TriangleList),
        Mesh::new(PrimitiveTopology::TriangleList),
        Mesh::new(PrimitiveTopology::TriangleList),
        Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
        // Mesh::new(PrimitiveTopology::TriangleList),
    ];

    // vx.topology.set(uvec3(0, 8, 0), true);
    info!("Converting minecraft mca map into plat");

    //let mut plat = Plat::load_mca("./assets/mca/121/", (0..11, 0..11)).unwrap();
    let mut plat = VenxPlat::new(12, 5, 9);

    //plat.load("saves/25_typed.plat");

    plat.set_voxel(0, (4, 4, 4).into(), 1);
    plat.set_voxel(0, (4, 5, 4).into(), 1);
    plat.set_voxel(0, (5, 5, 5).into(), 2);

    let capacity = 350_000;
    // panic!();
    let start = Instant::now();
    let mut mshs = vec![
        (
            Vec::<Vec3>::with_capacity(capacity),
            Vec::<Vec4>::with_capacity(capacity),
            Vec::<Vec3>::with_capacity(capacity)
        );
        meshes.len()
    ];
    log::info!("Loading chunks and computing mesh");

    let chunks_width = 1 * 1;
    let chunk_start = 0;
    let mut counter = 0;
    for x in 0..1 {
        info!("Progress: {}/{}", x, chunks_width);
        for z in 0..1 {
            for y in (0..1) {
                let time = Instant::now();
                let mut lod_level = (u32::max(z, x) / 128) as u8;

                if lod_level > 2 {
                    lod_level = 2;
                }

                lod_level = 0;
                dbg!("Load chunk");
                let chunk = plat.load_chunk(uvec3(x, y, z), 0);
                dbg!("Compute mesh");
                let vx_mesh = plat.compute_mesh_from_chunk(&chunk);
                // eprintln!("{chunk:?}");
                let idx = counter / capacity;

                for (pos, color, normal) in vx_mesh {
                    let new_pos: bevy::prelude::Vec3 =
                        bevy::prelude::Vec3::from_array(pos.to_array());
                    let new_color: bevy::prelude::Vec4 =
                        bevy::prelude::Vec4::from_array(color.to_array());
                    let new_normal: bevy::prelude::Vec3 =
                        bevy::prelude::Vec3::from_array(normal.to_array());

                    counter += 1;
                    mshs[idx].0.push(new_pos);
                    mshs[idx].1.push(new_color);
                    mshs[idx].2.push(new_normal);
                }
            }
        }
    }

    //   panic!();

    log::info!("finish loading and computing mesh");

    for msh in &mshs {
        //   dbg!("Vertices: ", msh.0.len());
    }

    dbg!(mshs[0].0.len());

    for (i, mesh) in meshes.iter_mut().enumerate() {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mshs[i].0.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, mshs[i].1.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mshs[i].2.clone());

        cmd.spawn(PbrBundle {
            mesh: bevy_meshes.add(mesh.clone()),
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
}
