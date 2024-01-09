use std::time::{Duration, Instant};

use bevy::pbr::wireframe::Wireframe;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{log, prelude::*};
use glam::{uvec3, vec4};
use main::{plat::VenxPlat, Venx};
use venx::plat::Plat;
use venx::voxel::cpu::topology::graph::Graph;
use venx::voxel::cpu::traverse::TrProps;
use venx::voxel::cpu::voxel::Voxel;
use venx::voxel::interfaces::layer::LayerInterface;
use venx::voxel::interfaces::load::LoadInterface;
use venx::voxel::interfaces::voxel::VoxelInterface;
use venx::voxel::segment::{Segment, SegmentStatic};

mod main;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Venx, LogDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}
fn setup(
    mut cmd: Commands,
    mut q: Query<&mut VenxPlat>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // let mut vx = Voxel::new(10, 4, 7);

    // vx.topology.set(uvec3(0, 1, 0), true);
    // vx.topology.set(uvec3(0, 2, 0), true);
    // vx.topology.set(uvec3(1, 3, 0), true);
    // // second chunk
    info!("Starting the program");
    // vx.topology.set(uvec3(0, 8, 0), true);
    info!("Converting minecraft mca map into plat");

    //let mut plat = Plat::load_mca("./assets/mca/121/", (0..11, 0..11)).unwrap();
    let mut plat = Plat::new(13, 4, 9);
    // let mut node_counter = 0;
    // let mut empty_counter = 0;

    // for node in &v.layers[0].graph.levels[2].nodes {
    //     if node.ident == -1 {
    //         empty_counter += 1;
    //     } else {
    //         node_counter += 1;
    //     }
    // }
    // dbg!(node_counter, empty_counter);

    plat.load("saves/121_typed.plat");

    // panic!();
    let start = Instant::now();

    let mut bevy_mesh: Vec<Vec3> = vec![];
    let mut bevy_color: Vec<Vec4> = vec![];
    let mut bevy_normal: Vec<Vec3> = vec![];

    // let mut final_chunk = None;
    log::info!("Loading chunks and computing mesh");

    let chunks_width = 32 * 11;
    let chunk_start = 1;

    for x in chunk_start..chunks_width {
        info!("Progress: {}/{}", x, chunks_width);
        for z in chunk_start..chunks_width {
            for y in (7..13).rev() {
                let time = Instant::now();
                let mut lod_level = (u32::max(z, x) / 20) as u8;

                if lod_level > 2 {
                    lod_level = 2;
                }
                //lod_level = 0;
                let chunk = plat
                    .controller
                    .get_voxel()
                    .load_chunk(uvec3(x, y, z), lod_level);
                let vx_mesh = plat.controller.get_voxel().compute_mesh_from_chunk(&chunk);
                for (pos, color, normal) in vx_mesh {
                    let new_pos: bevy::prelude::Vec3 =
                        bevy::prelude::Vec3::from_array(pos.to_array());
                    let new_color: bevy::prelude::Vec4 =
                        bevy::prelude::Vec4::from_array(color.to_array());
                    let new_normal: bevy::prelude::Vec3 =
                        bevy::prelude::Vec3::from_array(normal.to_array());
                    bevy_mesh.push(new_pos);
                    bevy_color.push(new_color);
                    bevy_normal.push(new_normal);
                }

                //dbg!("Mesh generated in: {}", time.elapsed());
                // dbg!("Check");
                // chunk.iter(|p, t| {
                //     if t != 0 {
                //         // dbg!(p, t);
                //     }
                // });
                // panic!();

                // continue;
            }
        }
    }

    log::info!("finish loading and computing mesh");

    let len = bevy_mesh.len();

    dbg!("Vertices: ", len);

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, bevy_mesh);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, bevy_color);

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, bevy_normal);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; len]);
    // let plat = todo!();
    // cmd.spawn((VenxPlat(todo!())));
    // for plat in &mut q {
    //     let mut mtx = [[[0; 64]; 64]; 64];

    //     mtx[0][0][0] = 1;
    //     mtx[0][1][0] = 1;
    //     mtx[0][2][0] = 1;
    //     mtx[0][3][0] = 1;

    //     // let segment = SegmentStatic { mtx };

    //     // plat.0.insert_segment(segment, uvec3(0, 0, 0));

    //     // let chunk = plat.0.load_chunk(uvec3(0, 0, 0), 0);

    //     // let mesh = plat.0.compute_mesh();
    // }

    cmd.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1., 1., 1.),
            // alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        ..default()
    })
    //.insert(Wireframe)
    ;
}
