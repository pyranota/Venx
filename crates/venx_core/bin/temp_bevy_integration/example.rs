use bevy::prelude::*;
use glam::uvec3;
use main::{plat::VenxPlat, Venx};
use venx_core::voxel::segment::SegmentStatic;
use venx_core::voxel::vx_trait::*;

mod main;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Venx))
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut cmd: Commands, mut q: Query<&mut VenxPlat>) {
    // cmd.spawn((VenxPlat(todo!())));
    for plat in &mut q {
        let mut mtx = [[[0; 64]; 64]; 64];

        mtx[0][0][0] = 1;
        mtx[0][1][0] = 1;
        mtx[0][2][0] = 1;
        mtx[0][3][0] = 1;

        // let segment = SegmentStatic { mtx };

        // plat.0.insert_segment(segment, uvec3(0, 0, 0));

        // let chunk = plat.0.load_chunk(uvec3(0, 0, 0), 0);

        // let mesh = plat.0.compute_mesh();
    }
}
