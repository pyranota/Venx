#![no_std]

use spirv_std::{glam::UVec3, spirv};
use venx_core::plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat};

#[spirv(compute(threads(2)))]
pub fn load_chunk(
    #[spirv(global_invocation_id)] id: UVec3,
    // TODO: Write macro to improve it
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] meta: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] base_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] base_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] tmp_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 1)] tmp_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] schem_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 1)] schem_entries: &mut [usize],

    #[spirv(storage_buffer, descriptor_set = 4, binding = 0)] canvas_nodes: &mut [Node],
    #[spirv(storage_buffer, descriptor_set = 4, binding = 1)] canvas_entries: &mut [usize],
) {
    let mut plat = RawPlat {
        position: (0, 0, 0),
        rotation: (0, 0, 0),
        depth: meta[0] as u8,
        base: Layer {
            freezed: false,
            depth: meta[0] as u8,
            entries: base_entries,
            nodes: base_nodes,
        },
        tmp: Layer {
            freezed: false,
            depth: meta[0] as u8,
            entries: tmp_entries,
            nodes: tmp_nodes,
        },
        schem: Layer {
            freezed: false,
            depth: meta[0] as u8,
            entries: schem_entries,
            nodes: schem_nodes,
        },
        canvas: Layer {
            freezed: false,
            depth: meta[0] as u8,
            entries: canvas_entries,
            nodes: canvas_nodes,
        },
    };

    plat[0].set((0, 0, 0).into(), 1);
}
