#[cfg(feature = "mca_converter")]
mod mca_converter;
mod minecraft_blocks;

use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::{
    chunk::{chunk::Chunk, storage::ChunksStorage},
    controller::Controller,
    voxel::{
        cpu::{mesh::Mesh, voxel::Voxel},
        interfaces::{layer::LayerInterface, voxel::VoxelInterface},
        segment::Segment,
    },
};
// #[derive(Clone)]
pub struct Plat {
    pub controller: Controller,
    // chunks: ChunksStorage,
}

pub struct VenxPlat {
    plat: plats::plat::Plat,
}

impl std::fmt::Debug for Plat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plat")
            // .field("controller", &self.controller)
            // .field("chunks", &self.chunks)
            .finish()
    }
}

#[cfg(feature = "bevy_ecs")]
impl Component for Plat {
    type Storage;
}

impl Plat {
    /// Depth, chunk_level, segment_level
    pub fn new(depth: u8, chunk_level: u8, segment_level: u8) -> Self {
        Plat {
            controller: Controller::new(depth, chunk_level, segment_level),
            //chunks: ChunksStorage {},
        }
    }
    pub fn load(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let decoded: Voxel = bitcode::decode(&data).unwrap();
        let prev: &mut Voxel = self.controller.get_voxel_mut().downcast_mut().unwrap();
        *prev = decoded;

        Ok(())
    }
    pub fn save(&mut self, path: &str) -> std::io::Result<()> {
        let v: &mut Voxel = self.controller.get_voxel_mut().downcast_mut().unwrap();

        // Remove lookup hashmaps in layers
        v.layers[0].graph.lookup_levels.clear();

        let encoded: Vec<u8> = bitcode::encode(&*v).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }
    pub fn new_segment(&self) -> Segment {
        // Segment::new(self.se)
        todo!()
    }
}

// fn insert_segment(&mut self, segment: crate::voxel::segment::Segment, position: glam::UVec3) {
//     self.controller
//         .get_voxel_mut()
//         .insert_segment(segment, position);
// }

// impl VoxelInterface for Plat {
//     fn load_chunk(
//         &self,
//         position: glam::UVec3,
//         level: u8,
//     ) -> std::option::Option<crate::chunk::chunk::Chunk> {
//         self.controller.get_voxel().load_chunk(position, level)
//     }

//     fn load_chunks(&self, position: glam::UVec3, level: u8) -> crate::chunk::chunk::Chunk {
//         self.controller.get_voxel().load_chunks(position, level)
//     }

//     fn load_chunk_n_mesh(&self) {
//         self.controller.get_voxel().load_chunk_n_mesh()
//     }

//     fn load_chunks_n_meshes(&self) {
//         self.controller.get_voxel().load_chunks_n_meshes()
//     }

//     fn compute_mesh_from_chunk(&self, chunk: &Chunk) -> Mesh {
//         self.controller.get_voxel().compute_mesh_from_chunk(chunk)
//     }

//     fn get(&self, level: u8, position: glam::UVec3) -> Option<usize> {
//         self.controller.get_voxel().get(level, position)
//     }
// }
