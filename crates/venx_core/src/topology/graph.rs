use glam::UVec3;
use venx_compute::{BindGroupBuilder, ComputeServer};

use super::execloc::Execloc;

pub struct Graph {
    segment_size: u32,
    compression_level: u32,
}

type MeshSize = u32;
type Chunk = String;

#[cfg(feature = "gpu")]
impl Graph {
    pub async fn load_chunks_gpu<'a>(
        cs: &mut ComputeServer,
        chunks: &'a [(UVec3, u32)],
    ) -> Vec<(Chunk, MeshSize)> {
        // Preprocessing chunks to load

        let to_load_buf = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 5]));

        let affected_buffers = cs.new_buffer(contents);

        let meta_bg = BindGroupBuilder::new()
            .insert(0, false, to_load_buf.as_entire_binding())
            .insert(1, false, affected_buffers.as_entire_binding())
            .build(&cs);

        let chunks_32_buf = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 5]));

        let chunks_16_buf = cs.new_buffer(bytemuck::cast_slice(&[4_u32; 5]));

        let chunks_bg = BindGroupBuilder::new()
            .insert(0, false, chunks_32_buf.as_entire_binding())
            .insert(1, false, chunks_16_buf.as_entire_binding())
            .build(&cs);

        let output_32_buf = cs.new_staging_buffer(4 * 5, true);

        let output_16_buf = cs.new_staging_buffer(4 * 5, true);
        todo!()
    }

    pub async fn load_chunk_gpu<'a>(
        cs: &mut ComputeServer,
        position: UVec3,
        level: u8,
    ) -> Vec<(Chunk, MeshSize)> {
        // Preprocessing chunks to load
        todo!()
    }

    pub async fn turbo_load_chunks_gpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<Chunk> {
        todo!()
    }

    pub async fn set_unmerged_gpu() {}

    pub async fn set_gpu() {}

    pub async fn load_meshes_gpu<'a>(
        chunks: Vec<Chunk>,
        mesh_sizes: Vec<MeshSize>,
        positions: Vec<UVec3>,
    ) -> Vec<Vec<UVec3>> {
        todo!()
    }

    /// Merging nodes inside **Segment**
    pub async fn local_merge_gpu() {}

    /// Looking from all nodes perspective and merging the given segment
    pub async fn global_merge_gpu() {}

    pub async fn complete_segment_gpu(
        cs: &mut ComputeServer,
        segment: Vec<Vec<Vec<u32>>>,
        position: UVec3,
    ) {
    }
}

impl Graph {
    pub async fn load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<(Chunk, MeshSize)> {
        todo!()
    }

    pub async fn turbo_load_chunks_cpu<'a>(level: u8, positions: &'a [UVec3]) -> Vec<Chunk> {
        todo!()
    }

    pub async fn set_voxel_cpu() {}

    pub async fn complete_segment_cpu(&mut self, segment: Vec<Vec<Vec<u32>>>, position: UVec3) {
        todo!()
    }
}
