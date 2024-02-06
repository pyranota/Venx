#![no_std]

use spirv_std::{glam::UVec3, spirv};
use venx_core::plat::node::Node;

#[spirv(compute(threads(32)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] base_nodes: &mut [Node],
) {
    base_nodes[0].children[4] = 4;
}
