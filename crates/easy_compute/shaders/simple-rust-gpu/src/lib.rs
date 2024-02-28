// #![cfg_attr(target_arch = "spirv", no_std)]
// // HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
// pub mod plats;
// use plats::{Level, Plat};
// use spirv_std::{glam::UVec3, spirv};

// #[spirv(compute(threads(32)))]
// pub fn plats_entry_full(
//     #[spirv(global_invocation_id)] id: UVec3,

//     // Base layer
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] layer_0_level_0: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] layer_0_level_1: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] layer_0_level_2: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] layer_0_level_3: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] layer_0_level_4: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] layer_0_level_5: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] layer_0_level_6: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] layer_0_level_7: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] layer_0_level_8: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 9)] layer_0_level_9: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 10)] layer_0_level_10: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 11)] layer_0_level_11: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] layer_0_level_12: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 13)] layer_0_level_13: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 14)] layer_0_level_14: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 15)] layer_0_level_15: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 16)] layer_0_level_16: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 17)] layer_0_level_17: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 18)] layer_0_level_18: &mut [u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 19)] layer_0_level_19: &mut [u32],
// ) {
// }
// #[spirv(compute(threads(32)))]
// plats_entry!(
//     // Name
//     plats,
//     (|p: &mut Plat| {
//         p.levels[0].nodes[0] = 999;
//         p.levels[1].nodes[0] = 111;
//     })
// );

// // #[spirv(compute(threads(32)))]
// // pub fn plats_entry(
// //     #[spirv(global_invocation_id)] id: UVec3,
// //     #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] layer_0_level_0: &mut [u32],
// //     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] layer_0_level_1: &mut [u32],
// //     // args!(0, 2),
// //     // Base layer
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] layer_0_level_0: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] layer_0_level_1: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] layer_0_level_2: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] layer_0_level_3: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] layer_0_level_4: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] layer_0_level_5: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] layer_0_level_6: &mut [u32],
// //     // #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] layer_0_level_7: &mut [u32],
// // ) {
// //     // Composing plat
// //     // Zerocost operation
// //     let plat = Plat {
// //         levels: [
// //             Level {
// //                 nodes: layer_0_level_0,
// //             },
// //             Level {
// //                 nodes: layer_0_level_1,
// //             },
// //         ],
// //     };
// //     plat.levels[0].nodes[0] = 999;
// //     plat.levels[1].nodes[0] = 111;
// //     // layer_0_level_1.nodes[0] = 999;
// //     // list[id.x as usize] = 111;
// // }

// // LocalSize/numthreads of (x = 64, y = 1, z = 1)
// #[spirv(compute(threads(32)))]
// pub fn main(
//     #[spirv(global_invocation_id)] id: UVec3,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] list: &mut [u32; 32],
// ) {
//     list[id.x as usize] = 999;
// }
