use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use glam::{uvec3, vec4};
use main::{plat::VenxPlat, Venx};
use venx_core::plat::Plat;
use venx_core::voxel::cpu::topology::graph::Graph;
use venx_core::voxel::cpu::voxel::Voxel;
use venx_core::voxel::segment::{Segment, SegmentStatic};

mod main;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Venx))
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
    // vx.topology.set(uvec3(0, 8, 0), true);
    let plat = Plat::load_mca("../../saves/mca/4region/", (-1..0, 0..1)).unwrap();
    // let mut plat = Plat::new(6, 4, 5);
    // // let mut plat = Plat::new(3, 2, 2);
    // plat.controller
    //     .get_voxel_mut()
    //     .set_voxel(0, (1, 1, 1).into(), 1);

    // plat.controller
    //     .get_voxel_mut()
    //     .set_voxel(0, (1, 2, 1).into(), 2);

    // let chunk = plat.controller.get_voxel().load_chunk((0, 0, 0).into());

    // let

    // dbg!(chunk.get((1, 1, 1)));

    // panic!();

    // let mut segment = Segment::new(5);
    // let red = 3;
    // let white = 1;
    // let green = 2;
    // let blue = 4;

    // segment.set((1, 0, 1), blue);
    // segment.set((1, 2, 1), white);
    // segment.set((1, 1, 1), red);
    // segment.set((0, 1, 0), green);
    // segment.set((0, 3, 0), blue);
    // segment.set((0, 0, 0), blue);
    // segment.set((0, 0, 1), blue);
    // segment.set((0, 0, 2), green);

    // plat.insert_segment(segment, (0, 0, 0).into());

    // dbg!(&plat.controller.get_voxel());

    let mut bevy_mesh: Vec<Vec3> = vec![];
    let mut bevy_color: Vec<Vec4> = vec![];

    // let mut final_chunk = None;
    log::info!("Loading chunks and computing mesh");

    for x in 0..18 {
        for z in 0..18 {
            for y in (0..10).rev() {
                let chunk = plat.controller.get_voxel().load_chunk(uvec3(x, y, z));
                let vx_mesh = plat.controller.get_voxel().compute_mesh_from_chunk(&chunk);
                // dbg!("Check");
                // chunk.iter(|p, t| {
                //     if t != 0 {
                //         dbg!(p, t);
                //     }
                // });
                // panic!();
                for (pos, color) in vx_mesh {
                    bevy_mesh.push(pos);
                    bevy_color.push(color);
                }
                // continue;
            }
        }
    }

    log::info!("finish loading and computing mesh");

    let len = bevy_mesh.len();

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, bevy_mesh);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, bevy_color);

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; len]);
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
    });
}
