use std::borrow::{Borrow, BorrowMut};

use log::trace;
use venx_core::{glam::*, plat::chunk::chunk::Chunk, utils::l2s};

use super::cpu_plat::CpuPlat;

pub const MESH_SIZE: usize = 36_000;
pub const CHUNK_BUCKET: usize = 256;

pub type Mesh = Box<Vec<[f32; 10]>>; // Position, Color, Normal

impl CpuPlat {
    pub fn to_mesh_greedy(&self, chunk: &Chunk) -> Mesh {
        trace!("to_mesh_greedy was called");
        trace!("Allocate box with mesh of size {MESH_SIZE}");
        let mut mesh_box = Box::new(vec![[0.; 10]; MESH_SIZE]);
        let mut mesh_idx = 0;
        let mesh = &mut *mesh_box;

        //let scale = 1.;
        //let scale = 1.;

        let lod_level = chunk.lod_level();

        trace!("Allocated 6 chunk mesh helpers");

        let mut mesh_helper_up = Chunk::new(UVec3::ZERO, lod_level, 5);
        let mut mesh_helper_down = Chunk::new(UVec3::ZERO, lod_level, 5);
        let mut mesh_helper_front = Chunk::new(UVec3::ZERO, lod_level, 5);
        let mut mesh_helper_back = Chunk::new(UVec3::ZERO, lod_level, 5);
        let mut mesh_helper_left = Chunk::new(UVec3::ZERO, lod_level, 5);
        let mut mesh_helper_right = Chunk::new(UVec3::ZERO, lod_level, 5);

        trace!("Iterating over all chunk");
        chunk.iter(|pos, block| {
            if block != 0 {
                let block_color = match block {
                    1 => ivec3(111, 54, 55),           // Dirt
                    2 | 17 => ivec3(93, 189, 101),     // Grass
                    3 | 5 | 6 => ivec3(213, 213, 213), // Stone + Diorite + Andesite
                    4 => ivec3(255, 155, 155),         // Granite
                    7 => ivec3(0, 0, 0),               // Bedrock
                    8 => ivec3(131, 162, 255),         // Water
                    9 => ivec3(186, 186, 186),         // Gravel
                    10 => ivec3(255, 214, 9),          // Gold ore
                    11 => ivec3(226, 226, 226),        // Iron ore
                    12 => ivec3(47, 47, 47),           // Coal ore
                    13 => ivec3(156, 81, 0),           // Oak log
                    14 => ivec3(0, 250, 33),           // Oak leaves
                    15 => ivec3(27, 96, 243),          // Lapis ore
                    16 => ivec3(245, 241, 169),        // Sand
                    18 => ivec3(116, 243, 255),        // Diamond ore
                    19 => ivec3(196, 151, 80),         // Birch log
                    20 => ivec3(60, 223, 83),          // Birch leaves
                    21 => ivec3(126, 51, 0),           // Dark Oak log
                    22 => ivec3(0, 223, 13),           // Dark Oak leaves
                    _ => ivec3(0, 0, 0),               // Else
                };

                let block_color = block_color.as_vec3().extend(0.5) / vec4(256., 256., 256., 1.0);

                // TOP
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_up,
                    &chunk,
                    block,
                    pos,
                    0,
                    2,
                    ivec3(0, 1, 0),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::TOP,
                );

                // BOTTOM
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_down,
                    &chunk,
                    block,
                    pos,
                    0,
                    2,
                    ivec3(0, -1, 0),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::BOTTOM,
                );

                // LEFT
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_left,
                    &chunk,
                    block,
                    pos,
                    2,
                    1,
                    ivec3(-1, 0, 0),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::LEFT,
                );

                // RIGHT
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_right,
                    &chunk,
                    block,
                    pos,
                    2,
                    1,
                    ivec3(1, 0, 0),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::RIGHT,
                );

                // FRONT
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_front,
                    &chunk,
                    block,
                    pos,
                    0,
                    1,
                    ivec3(0, 0, 1),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::FRONT,
                );

                // BACK
                self.borrow_raw_plat().greedy_runner(
                    &mut mesh_helper_back,
                    &chunk,
                    block,
                    pos,
                    0,
                    1,
                    ivec3(0, 0, -1),
                    mesh,
                    &mut mesh_idx,
                    block_color,
                    cube::BACK,
                );
            }
        });
        // dbg!(mesh_idx);
        trace!("Return mesh");
        mesh_box
    }
}

// impl Voxel {
//     /// No side culling
//     pub fn to_mesh_no_culling(&self, chunk: &Chunk) -> Mesh {
//         let mut mesh = vec![];

//         chunk.iter(|pos, block| {
//             if block != 0 {
//                 let cube = cube::FULL;
//                 for vertex in cube {
//                     mesh.push(((vertex + pos.as_vec3()), vec4(1., 1., 1., 1.), Vec3::ZERO))
//                 }
//             }
//         });
//         mesh
//     }
//     /// Fast way to do it, with side culling
//     pub fn to_mesh_naive(&self, chunk: &Chunk) -> Mesh {
//         let mut mesh = vec![];

//         chunk.iter(|pos, block| {
//             if block != 0 {
//                 let block_color = match block {
//                     1 => ivec3(111, 54, 55),           // Dirt
//                     2 | 17 => ivec3(93, 189, 101),     // Grass
//                     3 | 5 | 6 => ivec3(213, 213, 213), // Stone + Diorite + Andesite
//                     4 => ivec3(255, 155, 155),         // Granite
//                     7 => ivec3(0, 0, 0),               // Bedrock
//                     8 => ivec3(131, 162, 255),         // Water
//                     9 => ivec3(186, 186, 186),         // Gravel
//                     10 => ivec3(255, 214, 9),          // Gold ore
//                     11 => ivec3(226, 226, 226),        // Iron ore
//                     12 => ivec3(47, 47, 47),           // Coal ore
//                     13 => ivec3(156, 81, 0),           // Oak log
//                     14 => ivec3(0, 163, 33),           // Oak leaves
//                     15 => ivec3(27, 96, 243),          // Lapis ore
//                     16 => ivec3(245, 241, 169),        // Sand
//                     18 => ivec3(116, 243, 255),        // Diamond ore
//                     19 => ivec3(196, 151, 80),         // Birch log
//                     20 => ivec3(60, 223, 83),          // Birch leaves
//                     21 => ivec3(126, 51, 0),           // Dark Oak log
//                     22 => ivec3(0, 143, 13),           // Dark Oak leaves
//                     _ => ivec3(0, 0, 0),               // Else
//                 };

//                 let scale = lvl_to_size(chunk.lod_level) as f32;
//                 // let scale = 1.;
//                 let scale2 = lvl_to_size(chunk.lod_level) as f32;
//                 let scale3 = scale2 as u32;
//                 //dbg!(scale2);

//                 let block_color = block_color.as_vec3().extend(1.) / vec4(256., 256., 256., 1.0);

//                 if self
//                     .get_neighbor(chunk, pos.as_ivec3(), (0, 1, 0))
//                     .is_none()
//                 {
//                     let cube = cube::TOP;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(0., 1., 0.),
//                         ))
//                     }
//                 } else {
//                 }
//                 if self
//                     .get_neighbor(chunk, pos.as_ivec3(), (0, -1, 0))
//                     .is_none()
//                 {
//                     let cube = cube::BOTTOM;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(0., -1., 0.),
//                         ))
//                     }
//                 }
//                 if self
//                     .get_neighbor(chunk, (pos).as_ivec3(), (1, 0, 0))
//                     .is_none()
//                 {
//                     let cube = cube::RIGHT;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(1., 0., 0.),
//                         ))
//                     }
//                 }
//                 if self
//                     .get_neighbor(chunk, (pos).as_ivec3(), (-1, 0, 0))
//                     .is_none()
//                 {
//                     let cube = cube::LEFT;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(-1., 0., 0.),
//                         ))
//                     }
//                 }
//                 if self
//                     .get_neighbor(chunk, (pos).as_ivec3(), (0, 0, 1))
//                     .is_none()
//                 {
//                     let cube = cube::FRONT;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(0., 0., 1.),
//                         ))
//                     }
//                 }
//                 if self
//                     .get_neighbor(chunk, (pos).as_ivec3(), (0, 0, -1))
//                     .is_none()
//                 {
//                     let cube = cube::BACK;
//                     for vertex in cube {
//                         mesh.push((
//                             (vertex * scale2
//                                 + (pos * scale3
//                                     + (chunk.position * chunk.size() * (scale as u32)))
//                                     .as_vec3()),
//                             block_color,
//                             vec3(0., 0., -1.),
//                         ))
//                     }
//                 }
//             }
//         });
//         mesh
//     }
// }

pub mod cube {
    use venx_core::glam::*;

    pub const FRONT: [Vec3; 6] = [
        Vec3::new(-0., -0., 1.0),
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., -0., 1.0),
    ];

    pub const BACK: [Vec3; 6] = [
        Vec3::new(1.0, -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, -0., -0.),
    ];

    pub const TOP: [Vec3; 6] = [
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, 1.0),
    ];

    pub const BOTTOM: [Vec3; 6] = [
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, -0., 1.0),
    ];

    pub const RIGHT: [Vec3; 6] = [
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -0., 1.0),
    ];

    pub const LEFT: [Vec3; 6] = [
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., -0., -0.),
    ];

    pub const FULL: [Vec3; 36] = [
        // front face
        Vec3::new(-0., -0., 1.0),
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., -0., 1.0),
        // back face
        Vec3::new(1.0, -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, -0., -0.),
        // top face
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, 1.0),
        // bottom face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, -0., 1.0),
        // right face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -0., 1.0),
        // left face
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., -0., -0.),
    ];
}
