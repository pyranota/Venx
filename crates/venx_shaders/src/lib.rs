#![cfg_attr(target_arch = "spirv", no_std)]
mod primitive;
use primitive::cube;
use spirv_std::{
    glam::{ivec3, uvec3, vec4, UVec3},
    spirv,
};
use venx_core::plat::{
    chunk::{
        self,
        chunk::{Chunk, ChunkLoadRequest, ChunkMeta},
    },
    layer::layer::Layer,
    node::{Node, NodeAddr},
    node_l2::NodeL2,
    raw_plat::{LayerIndex::Base, RawPlat},
};

#[spirv(compute(threads(1)))]
pub fn load_chunk(
    #[spirv(global_invocation_id)] id: UVec3,
    // TODO: Write macro to improve it
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] l2: &mut [NodeL2],

    #[spirv(storage_buffer, descriptor_set = 4, binding = 0)] meta: &mut [usize],
    #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunks: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 5, binding = 1)]
    chunks_requests: &[ChunkLoadRequest],
) {
    // let depth = meta[id.x as usize] as usize;

    let layer: Layer = (nodes, l2, 12).into();

    let chunk = &mut chunks[id.x as usize];

    chunk.clean();

    let request = chunks_requests[id.x as usize];

    chunk.update_meta(
        UVec3::from_array(request.position),
        request.lod_level as usize,
        request.chunk_level as usize,
    );

    let lod_level = chunk.lod_level();

    // for i in 6..(32 * 32 * 32 + 6) {
    //     let p = chunk.from_flatten(i);
    //     let voxel = layer.get_node_gpu_no_enum(p + (chunk.position() * chunk.width()), lod_level);
    //     chunk.set(p, voxel as u32);
    //     //chunk.data[i as usize] = voxel as u32;
    // }
    layer.load_chunk_gpu(&mut chunks[id.x as usize]);
}
// Per chunk basis
// pub const MESH_SIZE: usize = 39_000;

// #[spirv(compute(threads(1)))]
// pub fn to_mesh_greedy(
//     #[spirv(global_invocation_id)] id: UVec3,

//     // TODO: Write macro to improve it
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] base_nodes: &mut [Node],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] base_l2: &mut [usize],

//     #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] tmp_nodes: &mut [Node],
//     #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] tmp_l2: &mut [usize],

//     #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] schem_nodes: &mut [Node],
//     #[spirv(storage_buffer, descriptor_set = 2, binding = 1)] schem_l2: &mut [usize],

//     #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] canvas_nodes: &mut [Node],
//     #[spirv(storage_buffer, descriptor_set = 3, binding = 1)] canvas_l2: &mut [usize],

//     #[spirv(storage_buffer, descriptor_set = 4, binding = 0)] meta: &mut [usize],

//     #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunks: &[Chunk],

//     #[spirv(storage_buffer, descriptor_set = 6, binding = 0)] mesh: &mut [[f32; 10]],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 1)] mesh_helpers_up: &mut [Chunk],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 2)] mesh_helpers_down: &mut [Chunk],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 3)] mesh_helpers_left: &mut [Chunk],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 4)] mesh_helpers_right: &mut [Chunk],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 5)] mesh_helpers_front: &mut [Chunk],
//     #[spirv(storage_buffer, descriptor_set = 6, binding = 6)] mesh_helpers_back: &mut [Chunk],
// ) {
//     let plat = RawPlat {
//         position: (0, 0, 0),
//         rotation: (0, 0, 0),
//         depth: meta[0] as usize,
//         layers: [
//             Layer {
//                 freezed: true,
//                 depth: meta[0] as usize,
//                 entries: base_l2,
//                 nodes: base_nodes,
//             },
//             Layer {
//                 freezed: false,
//                 depth: meta[0] as usize,
//                 entries: tmp_l2,
//                 nodes: tmp_nodes,
//             },
//             Layer {
//                 freezed: false,
//                 depth: meta[0] as usize,
//                 entries: schem_l2,
//                 nodes: schem_nodes,
//             },
//             Layer {
//                 freezed: false,
//                 depth: meta[0] as usize,
//                 entries: canvas_l2,
//                 nodes: canvas_nodes,
//             },
//         ],
//     };

//     let id = id.x as usize;

//     let mut mesh_idx = id * MESH_SIZE;

//     let chunk = &chunks[id];

//     let mesh_helper_up = &mut mesh_helpers_up[id];
//     let mesh_helper_down = &mut mesh_helpers_down[id];
//     let mesh_helper_left = &mut mesh_helpers_left[id];
//     let mesh_helper_right = &mut mesh_helpers_right[id];
//     let mesh_helper_front = &mut mesh_helpers_front[id];
//     let mesh_helper_back = &mut mesh_helpers_back[id];

//     mesh_helper_up.clean();
//     mesh_helper_down.clean();
//     mesh_helper_left.clean();
//     mesh_helper_right.clean();
//     mesh_helper_front.clean();
//     mesh_helper_back.clean();

//     let size = chunk.size();
//     for index in 6..(size * size * size + 6) {
//         let voxel_id = chunk.data[index as usize];

//         if voxel_id != 0 {
//             let pos = chunk.from_flatten(index as u32);

//             //   let block_color = block_color.as_vec3().extend(0.5) / vec4(256., 256., 256., 1.0);
//             let block_color = vec4(1., 1., 1., 1.);
//             let block_color = vec4(1., 1., 1., 1.);

//             let block = voxel_id;

//             // TOP
//             plat.greedy_runner(
//                 mesh_helper_up,
//                 chunk,
//                 block,
//                 pos,
//                 0,
//                 2,
//                 ivec3(0, 1, 0),
//                 mesh,
//                 &mut mesh_idx,
//                 block_color,
//                 primitive::cube::TOP,
//             );

//             // // BOTTOM
//             // plat.greedy_runner(
//             //     mesh_helper_down,
//             //     &chunk,
//             //     block,
//             //     pos,
//             //     0,
//             //     2,
//             //     ivec3(0, -1, 0),
//             //     mesh,
//             //     &mut mesh_idx,
//             //     block_color,
//             //     cube::BOTTOM,
//             // );

//             // // LEFT
//             // plat.greedy_runner(
//             //     mesh_helper_left,
//             //     &chunk,
//             //     block,
//             //     pos,
//             //     2,
//             //     1,
//             //     ivec3(-1, 0, 0),
//             //     mesh,
//             //     &mut mesh_idx,
//             //     block_color,
//             //     cube::LEFT,
//             // );

//             // // RIGHT
//             // plat.greedy_runner(
//             //     mesh_helper_right,
//             //     &chunk,
//             //     block,
//             //     pos,
//             //     2,
//             //     1,
//             //     ivec3(1, 0, 0),
//             //     mesh,
//             //     &mut mesh_idx,
//             //     block_color,
//             //     cube::RIGHT,
//             // );

//             // // FRONT
//             // plat.greedy_runner(
//             //     mesh_helper_front,
//             //     &chunk,
//             //     block,
//             //     pos,
//             //     0,
//             //     1,
//             //     ivec3(0, 0, 1),
//             //     mesh,
//             //     &mut mesh_idx,
//             //     block_color,
//             //     cube::FRONT,
//             // );

//             // // BACK
//             // plat.greedy_runner(
//             //     mesh_helper_back,
//             //     &chunk,
//             //     block,
//             //     pos,
//             //     0,
//             //     1,
//             //     ivec3(0, 0, -1),
//             //     mesh,
//             //     &mut mesh_idx,
//             //     block_color,
//             //     cube::BACK,
//             // );
//         }
//     }
// }
