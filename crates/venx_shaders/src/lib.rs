#![no_std]

use spirv_std::{
    glam::{uvec3, UVec3},
    spirv,
};
use venx_core::plat::{layer::layer::Layer, node::Node, raw_plat::RawPlat};

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
) {
    let mut plat = RawPlat {
        position: (0, 0, 0),
        rotation: (0, 0, 0),
        depth: meta[0] as usize,
        base: Layer {
            freezed: false,
            depth: meta[0] as usize,
            entries: base_entries,
            nodes: base_nodes,
        },
        tmp: Layer {
            freezed: false,
            depth: meta[0] as usize,
            entries: tmp_entries,
            nodes: tmp_nodes,
        },
        schem: Layer {
            freezed: false,
            depth: meta[0] as usize,
            entries: schem_entries,
            nodes: schem_nodes,
        },
        canvas: Layer {
            freezed: false,
            depth: meta[0] as usize,
            entries: canvas_entries,
            nodes: canvas_nodes,
        },
    };

    plat.load_chunk((0, 2, 0).into(), 0);

    // let mut layer = Layer {
    //     freezed: false,
    //     depth: meta[0] as usize,
    //     entries: base_entries,
    //     nodes: base_nodes,
    // };

    //plat[0].entry(1);
    // .test_entry_wrapper(1_u32 as usize);
    //plat.base.set(uvec3(0, 0, 0), 1);
    //let layer = &plat[0];
}
