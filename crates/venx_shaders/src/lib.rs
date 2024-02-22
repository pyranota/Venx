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
    op::{EntryOpts, LayerOpts},
    raw_plat::{LayerIndex::Base, RawPlat},
};

#[spirv(compute(threads(1)))]
pub fn load_chunk(
    #[spirv(global_invocation_id)] id: UVec3,
    // constants: PushConstant<u32>,
    // TODO: Write macro to improve it
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] base_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] base_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] tmp_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] tmp_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] schem_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 1)] schem_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] canvas_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 1)] canvas_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 4, binding = 0)] meta: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunks: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 5, binding = 1)]
    chunks_requests: &[ChunkLoadRequest],
) {
    let plat = RawPlat {
        position: (0, 0, 0),
        rotation: (0, 0, 0),
        depth: meta[0] as usize,
        layers: [
            Layer {
                freezed: false,
                depth: meta[0] as usize,
                entries: base_entries,
                nodes: base_nodes,
            },
            Layer {
                freezed: false,
                depth: meta[0] as usize,
                entries: tmp_entries,
                nodes: tmp_nodes,
            },
            Layer {
                freezed: false,
                depth: meta[0] as usize,
                entries: schem_entries,
                nodes: schem_nodes,
            },
            Layer {
                freezed: false,
                depth: meta[0] as usize,
                entries: canvas_entries,
                nodes: canvas_nodes,
            },
        ],
    };

    chunks[id.x as usize].clean();

    // let request = chunks_requests[id.x as usize];

    // chunks[id.x as usize].update_meta(
    //     UVec3::from_array(request.position),
    //     request.lod_level as usize,
    //     request.chunk_level as usize,
    // );

    // for idx in 7..chunk.data.len() {
    //     chunk.data[idx] = 9;
    // }
    // for x in 0..32 {
    //     for y in 0..32 {
    //         let mut z_s = [0; 32];
    //         for z in 0..32 {
    //             let _ = plat[0].get_node_gpu_no_enum((x, y, z).into(), 0);

    //             z_s[z as usize] = 4;
    //         }

    //         for z in 0..32 {
    //             chunk.set((x, y, z).into(), 2);
    //         }
    //     }
    // }

    // for x in 0..32 {
    //     for y in 0..32 {
    //         for z in 0..32 {
    //             chunks[id.x as usize].set((x, y, z).into(), 2);
    //         }
    //     }
    // }
    //let chunk = &mut chunks[id.x as usize];
    // for x in 0..32 {
    //     for y in 0..32 {
    //         for z in 0..32 {
    //             chunk.set((x, y, z).into(), 2);
    //         }
    //     }
    // }

    // for x in 0..32 {
    //     for y in 0..32 {
    //         for z in 0..32 {
    //             chunk.set((x, y, z).into(), 2);
    //         }
    //     }
    // }

    // // for x in 0..32 {
    // //     for y in 0..32 {
    // //         for z in 0..32 {
    // //
    // //         }
    // //     }
    // // }

    // // chunk.set((0, 0, 0).into(), 2);

    // for x in 0..32 {
    //     for y in 0..32 {
    //         for z in 0..32 {
    //             let _ = plat[0].get_node_gpu_no_enum((x, y, z).into(), 6);
    //         }
    //     }
    // }

    // plat.load_chunk_gpu(&mut chunks[id.x as usize]);

    //plat[0].traverse(0, 2, UVec3::ZERO, false, plat.depth, &mut |p| {});
    // let xm = 0;
    // let ym = (id.y - 1) * 16;
    // let zm = (id.z - 1) * 16;
    // //plat.load_chunk_gpu(&mut chunks[id.x as usize]);
    // let chunk = &mut chunks[id.x as usize / 2];
    // for x in 0..32 {
    //     for y in 0..16 {
    //         for z in 0..16 {
    //             // let addr = NodeAddr::from_position(
    //             //     uvec3(
    //             //         x + position.x * 32,
    //             //         y + position.y * 32,
    //             //         z + position.z * 32,
    //             //     ),
    //             //     self.depth,
    //             //     0,
    //             // );
    //             let voxel_id = plat[0].get_node_gpu(
    //                 uvec3(
    //                     x + chunk.position().x * 32 + xm,
    //                     y + chunk.position().y * 32 + ym,
    //                     z + chunk.position().z * 32 + zm,
    //                 ),
    //                 0,
    //                 None,
    //             );

    //             // let res = self.get_voxel();

    //             if voxel_id != 0 {
    //                 chunk.set((x + xm, y + ym, z + zm).into(), voxel_id as u32);
    //                 //chunk[0].set(uvec3(x, y, z), res.voxel_id as u32);
    //             }
    //         }
    //     }
    // }
}
/// Per chunk basis
pub const MESH_SIZE: usize = 36_000;

#[spirv(compute(threads(1)))]
pub fn to_mesh_greedy(
    #[spirv(global_invocation_id)] id: UVec3,

    // TODO: Write macro to improve it
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] base_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] base_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] tmp_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] tmp_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] schem_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 1)] schem_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] canvas_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 1)] canvas_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 4, binding = 0)] meta: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunks: &[Chunk],

    #[spirv(storage_buffer, descriptor_set = 6, binding = 0)] mesh: &mut [[f32; 10]],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 1)] mesh_helpers_up: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 2)] mesh_helpers_down: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 3)] mesh_helpers_left: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 4)] mesh_helpers_right: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 5)] mesh_helpers_front: &mut [Chunk],
    #[spirv(storage_buffer, descriptor_set = 6, binding = 6)] mesh_helpers_back: &mut [Chunk],
) {
    // let mut plat = RawPlat {
    //     position: (0, 0, 0),
    //     rotation: (0, 0, 0),
    //     depth: meta[0] as usize,
    //     layers: [
    //         Layer {
    //             freezed: false,
    //             depth: meta[0] as usize,
    //             entries: base_entries,
    //             nodes: base_nodes,
    //         },
    //         Layer {
    //             freezed: false,
    //             depth: meta[0] as usize,
    //             entries: tmp_entries,
    //             nodes: tmp_nodes,
    //         },
    //         Layer {
    //             freezed: false,
    //             depth: meta[0] as usize,
    //             entries: schem_entries,
    //             nodes: schem_nodes,
    //         },
    //         Layer {
    //             freezed: false,
    //             depth: meta[0] as usize,
    //             entries: canvas_entries,
    //             nodes: canvas_nodes,
    //         },
    //     ],
    // };

    // let id = id.x as usize;

    // let mut mesh_idx = id * MESH_SIZE;

    // let chunk = &chunks[id];

    // let lod_level = chunk.lod_level();

    // let mesh_helper_up = &mut mesh_helpers_up[id];
    // let mesh_helper_down = &mut mesh_helpers_down[id];
    // let mesh_helper_left = &mut mesh_helpers_left[id];
    // let mesh_helper_right = &mut mesh_helpers_right[id];
    // let mesh_helper_front = &mut mesh_helpers_front[id];
    // let mesh_helper_back = &mut mesh_helpers_back[id];

    // mesh_helper_up.clean();
    // mesh_helper_down.clean();
    // mesh_helper_left.clean();
    // mesh_helper_right.clean();
    // mesh_helper_front.clean();
    // mesh_helper_back.clean();

    // let size = chunk.size();
    // for index in 6..(size * size * size + 6) {
    //     let voxel_id = chunk.data[index as usize];

    //     if voxel_id != 0 {
    //         let pos = chunk.from_flatten(index as u32);

    //         //   let block_color = block_color.as_vec3().extend(0.5) / vec4(256., 256., 256., 1.0);
    //         let block_color = vec4(1., 1., 1., 1.);
    //         let block_color = vec4(1., 1., 1., 1.);

    //         let block = voxel_id;

    //         // TOP
    //         plat.greedy_runner(
    //             mesh_helper_up,
    //             chunk,
    //             block,
    //             pos,
    //             0,
    //             2,
    //             ivec3(0, 1, 0),
    //             mesh,
    //             &mut mesh_idx,
    //             block_color,
    //             primitive::cube::TOP,
    //         );

    //         // // BOTTOM
    //         // plat.greedy_runner(
    //         //     mesh_helper_down,
    //         //     &chunk,
    //         //     block,
    //         //     pos,
    //         //     0,
    //         //     2,
    //         //     ivec3(0, -1, 0),
    //         //     mesh,
    //         //     &mut mesh_idx,
    //         //     block_color,
    //         //     cube::BOTTOM,
    //         // );

    //         // // LEFT
    //         // plat.greedy_runner(
    //         //     mesh_helper_left,
    //         //     &chunk,
    //         //     block,
    //         //     pos,
    //         //     2,
    //         //     1,
    //         //     ivec3(-1, 0, 0),
    //         //     mesh,
    //         //     &mut mesh_idx,
    //         //     block_color,
    //         //     cube::LEFT,
    //         // );

    //         // // RIGHT
    //         // plat.greedy_runner(
    //         //     mesh_helper_right,
    //         //     &chunk,
    //         //     block,
    //         //     pos,
    //         //     2,
    //         //     1,
    //         //     ivec3(1, 0, 0),
    //         //     mesh,
    //         //     &mut mesh_idx,
    //         //     block_color,
    //         //     cube::RIGHT,
    //         // );

    //         // // FRONT
    //         // plat.greedy_runner(
    //         //     mesh_helper_front,
    //         //     &chunk,
    //         //     block,
    //         //     pos,
    //         //     0,
    //         //     1,
    //         //     ivec3(0, 0, 1),
    //         //     mesh,
    //         //     &mut mesh_idx,
    //         //     block_color,
    //         //     cube::FRONT,
    //         // );

    //         // // BACK
    //         // plat.greedy_runner(
    //         //     mesh_helper_back,
    //         //     &chunk,
    //         //     block,
    //         //     pos,
    //         //     0,
    //         //     1,
    //         //     ivec3(0, 0, -1),
    //         //     mesh,
    //         //     &mut mesh_idx,
    //         //     block_color,
    //         //     cube::BACK,
    //         // );
    //     }
    // }

    // // chunk.iter(|pos, block| {
    // //     if block != 0 {
    // //         // let block_color = match block {
    // //         //     1 => [111, 54, 55],                // Dirt
    // //         //     2 | 17 => ivec3(93, 189, 101),     // Grass
    // //         //     3 | 5 | 6 => ivec3(213, 213, 213), // Stone + Diorite + Andesite
    // //         //     4 => ivec3(255, 155, 155),         // Granite
    // //         //     7 => ivec3(0, 0, 0),               // Bedrock
    // //         //     8 => ivec3(131, 162, 255),         // Water
    // //         //     9 => ivec3(186, 186, 186),         // Gravel
    // //         //     10 => ivec3(255, 214, 9),          // Gold ore
    // //         //     11 => ivec3(226, 226, 226),        // Iron ore
    // //         //     12 => ivec3(47, 47, 47),           // Coal ore
    // //         //     13 => ivec3(156, 81, 0),           // Oak log
    // //         //     14 => ivec3(0, 250, 33),           // Oak leaves
    // //         //     15 => ivec3(27, 96, 243),          // Lapis ore
    // //         //     16 => ivec3(245, 241, 169),        // Sand
    // //         //     18 => ivec3(116, 243, 255),        // Diamond ore
    // //         //     19 => ivec3(196, 151, 80),         // Birch log
    // //         //     20 => ivec3(60, 223, 83),          // Birch leaves
    // //         //     21 => ivec3(126, 51, 0),           // Dark Oak log
    // //         //     22 => ivec3(0, 223, 13),           // Dark Oak leaves
    // //         //     _ => ivec3(0, 0, 0),               // Else
    // //         // };

    // //         //       mesh_helper_up.set(uvec3(0, 0, 0), 10);

    // //         //let block_color = block_color.as_vec3().extend(0.5) / vec4(256., 256., 256., 1.0);

    // //     }
    // // });
    // // trace!("Return mesh");
    // // mesh_box

    // //plat[Base].set((0, 5, 0).into(), 7);
    // // chunks[0].set((0, 0, 0).into(), 1);
    // // chunks[0].set((0, 1, 0).into(), 2);
    // // chunks[0].set((1, 0, 0).into(), 3);
    // // chunks[0].set((0, 0, 1).into(), 4);
    // // plat.load_chunk_gpu(&mut chunks[id.x as usize]);

    // // {
    // //     // plat[0].set(uvec3(0, 2, 2), 222);

    // // }
    // // drop(plat);
    // // {
    // //     chunk_flatten[0] = 0;
    // // }
}
