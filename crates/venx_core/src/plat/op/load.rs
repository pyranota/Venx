use spirv_std::glam::{uvec3, UVec3};

use crate::{
    plat::{
        chunk::chunk::{Chunk, ChunkMeta},
        node::NodeAddr,
        raw_plat::RawPlat,
    },
    utils::l2s,
};

use super::LayerOpts;

impl RawPlat<'_> {
    //#[cfg(not(feature = "bitcode_support"))]
    #[inline]
    pub fn load_chunk_gpu(
        &self,
        //chunk_meta: ChunkMeta,
        chunk: &mut Chunk,
    ) {
        // TODO change
        let chunk_level = 5;

        // let chunk_lod_scaler = l2s(lod_level);

        // // let real_chunk_size = l2s(chunk.level());

        // for x in 0..32 {
        //     for y in 0..32 {
        //         for z in 0..32 {
        //             let voxel_id = self[0].get_node_gpu(
        //                 uvec3(
        //                     x + chunk.position().x * 32,
        //                     y + chunk.position().y * 32,
        //                     z + chunk.position().z * 32,
        //                 ),
        //                 0,
        //                 None,
        //             );

        //             // let res = self.get_voxel();

        //             if voxel_id != 0 {
        //                 chunk.set((x, y, z).into(), voxel_id as u32);
        //                 //chunk[0].set(uvec3(x, y, z), res.voxel_id as u32);
        //             }
        //         }
        //     }
        // }

        //for layer_idx in 0..4 {
        let node_idx = self[0].get_node_idx_gpu(
            chunk.position() * l2s(chunk.chunk_level()),
            chunk.chunk_level(),
        );

        // let mut counter = 0;
        // const LIMIT: usize = 128;

        // let mut buffer = [0; LIMIT];

        if node_idx != 0 {
            self[0].traverse_gpu(
                0,
                node_idx,
                UVec3::ZERO,
                true,
                chunk.chunk_level(),
                |(level, entry, p)| {
                    if level == 0 {
                        if entry != 0 {
                            // buffer[counter] = entry as u32;

                            // counter += 1;
                            // if counter == LIMIT {
                            //     counter = 0;
                            //     chunk.set_many(buffer);
                            // }

                            //chunk.get(p);

                            chunk.set(p, entry as u32);
                            //chunk.data[100] = entry as u32;
                            //chunk.data[5] = entry as u32;
                            // a[p.x as usize] = chunk.chunk_level();
                            // b[p.y as usize] = chunk.chunk_level();
                            // c[p.z as usize] = chunk.chunk_level();

                            //let l = ;
                        }

                        //     // p.drop_tree = true;
                    }
                },
            );
        }
        //}

        // chunk.data[5] = 9;
    }
    //#[cfg(feature = "bitcode_support")]
    pub fn load_chunk(&self, position: UVec3, lod_level: usize) -> Chunk {
        // TODO change
        let chunk_level = 5;

        let chunk_lod_scaler = l2s(lod_level);

        let mut chunk = Chunk::new(position, lod_level, chunk_level);

        // // let real_chunk_size = l2s(chunk.level());

        // for x in 0..32 {
        //     for y in 0..32 {
        //         for z in 0..32 {
        //             let res = self.get_voxel(uvec3(
        //                 x + position.x * 32,
        //                 y + position.y * 32,
        //                 z + position.z * 32,
        //             ));

        //             if res.is_some() {
        //                 chunk.set(uvec3(x, y, z), res.voxel_id as u32);
        //             }
        //         }
        //     }
        // }

        self.traverse_region(
            position,
            chunk_level,
            super::EntryOpts::All,
            LayerOpts::All,
            &mut |props| {
                if props.level == lod_level {
                    chunk.set(*props.position / chunk_lod_scaler, props.entry);
                    props.drop_tree = true;
                }
            },
        );

        chunk
    }
}
#[cfg(feature = "bitcode_support")]
#[cfg(test)]
mod tests {
    use std::println;

    use alloc::vec;
    use spirv_std::glam::uvec3;

    use crate::plat::{node::Node, raw_plat::RawPlat};

    extern crate alloc;
    extern crate std;
    #[test]
    fn load_chunk() {
        let mut base = ([Node::default(); 128], [0; 10]);
        let (mut tmp, mut schem, mut canvas) = (base.clone(), base.clone(), base.clone());
        let mut plat = RawPlat::new(
            6,
            5,
            5,
            (&mut base.0, &mut base.1),
            (&mut tmp.0, &mut tmp.1),
            (&mut schem.0, &mut schem.1),
            (&mut canvas.0, &mut canvas.1),
        );
        plat[0].set(uvec3(15, 15, 15), 1);
        plat[0].set(uvec3(0, 0, 0), 2);

        let chunk = plat.load_chunk(uvec3(0, 0, 0), 0);

        //println!("{:?}", chunk);

        assert!(chunk.get(uvec3(0, 0, 0)).is_some());
    }
}
