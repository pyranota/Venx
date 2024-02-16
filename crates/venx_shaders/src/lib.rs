#![cfg_attr(target_arch = "spirv", no_std)]

use spirv_std::{
    glam::{uvec3, UVec3},
    spirv,
};
use venx_core::plat::{
    chunk::chunk::{Chunk, ChunkMeta},
    layer::layer::Layer,
    node::{Node, NodeAddr},
    op::{EntryOpts, LayerOpts},
    raw_plat::{LayerIndex::Base, RawPlat},
};

#[spirv(compute(threads(1)))]
pub fn load_chunk_2(
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

    #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunks: &mut [Chunk],
) {
    let mut plat = RawPlat {
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

    //plat[Base].set((0, 5, 0).into(), 7);
    // chunks[0].set((0, 0, 0).into(), 1);
    // chunks[0].set((0, 1, 0).into(), 2);
    // chunks[0].set((1, 0, 0).into(), 3);
    // chunks[0].set((0, 0, 1).into(), 4);
    plat.load_chunk_gpu(&mut chunks[id.x as usize]);

    // {
    //     // plat[0].set(uvec3(0, 2, 2), 222);

    // }
    // drop(plat);
    // {
    //     chunk_flatten[0] = 0;
    // }
}

#[spirv(compute(threads(1)))]
pub fn load_chunk(
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

    #[spirv(storage_buffer, descriptor_set = 5, binding = 0)] chunk_meta: &mut [ChunkMeta],
    #[spirv(storage_buffer, descriptor_set = 5, binding = 1)] chunk_flatten: &mut [u32],
) {
}
