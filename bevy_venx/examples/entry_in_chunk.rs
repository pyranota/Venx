/*
    This example aimed to find out what avg amount types of blocks in simple 32x32x32 chunk in mca world

*/

use std::f32::consts::PI;

use bevy::{
    math::vec3,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    utils::hashbrown::{HashMap, HashSet},
};
use bevy_panorbit_camera::PanOrbitCamera;
use venx::plat::{interfaces::layer::LayerInterface, VenxPlat};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_panorbit_camera::PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.52, 0.80, 0.92)))
        .run();
}
// 32x32x32
// fn setup(
//     mut cmd: Commands,
//     mut bevy_meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Its small-sized plat, its slow to convert it from mca each run, it will be saved
//     let plat = VenxPlat::load("mca_small_all_blocks").unwrap_or_else(|e| {
//         warn!("Plat wasnt found on device, creating new and saving ({e})");
//         // Convert from minecraft map
//         let plat = VenxPlat::load_mca_untyped("./assets/mca/1/", (0..1, 0..1)).unwrap();
//         plat.save("mca_small_all_blocks").unwrap();
//         plat
//     });
//     let mut map = HashMap::new();
//     for cx in 9..16 {
//         for cz in 9..16 {
//             for cy in 3..7 {
//                 let mut set = HashSet::new();
//                 info!("Iterating over chunk ({cx},{cy},{cz})");
//                 for bx in 0..32 {
//                     for bz in 0..32 {
//                         for by in 0..32 {
//                             let entry =
//                                 plat.get_voxel((bx + cx * 32, by + cy * 32, bz + cz * 32).into());

//                             set.insert(entry);
//                         }
//                     }
//                 }

//                 if set.len() > 1 {
//                     map.insert((cx, cy, cz), set.len());
//                 }
//             }
//         }
//     }

//     info!("{map:?}");

//     let mut total = 0;

//     let mut len = map.len();

//     for (_pos, key) in map.iter() {
//         total += key - 1;
//     }

//     let avg = total / len;

//     info!("AVG amount different types of blocks in single 32x32x32 chunk: {avg}");
// }

const WIDTH: usize = 16;

// // 16x16x16
fn setup(
    mut cmd: Commands,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Its small-sized plat, its slow to convert it from mca each run, it will be saved
    let plat = VenxPlat::load("mca_small_all_blocks").unwrap_or_else(|e| {
        warn!("Plat wasnt found on device, creating new and saving ({e})");
        // Convert from minecraft map
        let plat = VenxPlat::load_mca_untyped("./assets/mca/1/", (0..1, 0..1)).unwrap();
        plat.save("mca_small_all_blocks").unwrap();
        plat
    });
    let mut map = HashMap::new();
    for cy in 0..32 {
        for cz in 0..32 {
            for cx in 0..32 {
                let mut set = HashSet::new();
                info!("Iterating over chunk (y: {cy}, x: {cx}, z: {cz})");
                for bx in 0..16 {
                    for bz in 0..16 {
                        for by in 0..16 {
                            let entry =
                                plat.get_voxel((bx + cx * 16, by + cy * 16, bz + cz * 16).into());

                            set.insert(entry);
                        }
                    }
                }

                if set.len() > 1 {
                    map.insert((cx, cy, cz), set.len());
                }
            }
        }
    }

    info!("{map:?}");

    let mut total = 0;

    let mut len = map.len();

    for (_pos, key) in map.iter() {
        total += key - 1;
    }

    let avg = total / len;

    info!("AVG amount different types of blocks in single 16x16x16 chunk: {avg}");
}

// 8x8x8
// fn setup(
//     mut cmd: Commands,
//     mut bevy_meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Its small-sized plat, its slow to convert it from mca each run, it will be saved
//     let plat = VenxPlat::load("mca_small_all_blocks").unwrap_or_else(|e| {
//         warn!("Plat wasnt found on device, creating new and saving ({e})");
//         // Convert from minecraft map
//         let plat = VenxPlat::load_mca_untyped("./assets/mca/1/", (0..1, 0..1)).unwrap();
//         plat.save("mca_small_all_blocks").unwrap();
//         plat
//     });
//     let mut map = HashMap::new();
//     for cx in 36..64 {
//         for cz in 36..64 {
//             for cy in 10..28 {
//                 let mut set = HashSet::new();
//                 info!("Iterating over chunk ({cx},{cy},{cz})");
//                 for bx in 0..8 {
//                     for bz in 0..8 {
//                         for by in 0..8 {
//                             let entry =
//                                 plat.get_voxel((bx + cx * 8, by + cy * 8, bz + cz * 8).into());

//                             set.insert(entry);
//                         }
//                     }
//                 }

//                 if set.len() > 1 {
//                     map.insert((cx, cy, cz), set.len());
//                 }
//             }
//         }
//     }

//     info!("{map:?}");

//     let mut total = 0;

//     let mut len = map.len();

//     for (_pos, key) in map.iter() {
//         total += key - 1;
//     }

//     let avg = total / len;

//     info!("AVG amount different types of blocks in single 8x8x8 chunk: {avg}");
// }
