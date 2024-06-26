use anyhow::bail;
use glam::{UVec3, Vec3};

use crate::plat::normal::mesh::Mesh;

use super::{external_buffer::ExternalBuffer, BucketIdx};

pub type ExternalBufferObject = Box<dyn ExternalBuffer + 'static + Send + Sync>;

pub struct VertexPool {
    // TODO: Use linked list?
    pub free_buckets: Vec<BucketIdx>,
    /// Amount of vertices in single bucket
    pub bucket_size: u32,
    /// Amount of buckets
    pub bucket_amount: u32,
    // TODO: Write docs about it
    // TODO: Create abstraction
    // Smthg like Unit, where Unit = Bucket * BucketSize
    pub bucket_usage: Vec<u32>,
    pub indirect_buffer: ExternalBufferObject,
    pub vertex_buffer: ExternalBufferObject,
}

impl VertexPool {
    pub fn new(
        bucket_size: u32,
        bucket_amount: u32,
        bucket_usage: Vec<u32>,
        indirect_buffer: ExternalBufferObject,
        vertex_buffer: ExternalBufferObject,
    ) -> Self {
        assert_eq!(bucket_amount, bucket_usage.iter().sum::<u32>());
        assert_eq!(bucket_size % 6, 0);

        VertexPool {
            free_buckets: (0..bucket_amount)
                .map(|i| i as usize)
                .collect::<Vec<BucketIdx>>(),
            bucket_size,
            bucket_amount,
            indirect_buffer,
            vertex_buffer,
            bucket_usage,
        }
    }
    pub(super) fn allocate(&mut self, bucket_amount: u32) -> anyhow::Result<Vec<BucketIdx>> {
        let buckets = &mut self.free_buckets;
        let free = buckets.len() as u32;
        if free < bucket_amount {
            bail!(
                "
                You ran out of free buckets, cannot allocate anymore.
                Consider deallocating some before trying again \n
                Free buckets: {free:?}, Requested: {bucket_amount}
                "
            );
        }
        Ok(buckets.split_off(free as usize - bucket_amount as usize))
    }

    pub(super) fn deallocate(&mut self, mut buckets: Vec<BucketIdx>) {
        self.free_buckets.append(&mut buckets);
    }

    // TODO: Make gpu-friendly
    pub(super) fn load_mesh(&mut self, mesh: Vec<[f32; 3]>, buckets: Vec<BucketIdx>) {
        let bucket_size = self.bucket_size as usize;
        // Divide mesh in submeshes. Each one of them is size of single bucket
        // Iterate over submeshes and buckets at the same time
        // When buckets run out, we automatically exit this iteration
        // It leaves the rest of the mesh (which is potentially flooded with zeros)
        // We have allocated amount of buckets from other stages outside of this method
        // Which should be enough for entire mesh
        //
        // In other words, if submesh has atleast one non-zero vertex, it will be loaded
        for (submesh, bucket_idx) in mesh.chunks(bucket_size).zip(buckets.iter()) {
            // dbg!("Iter");
            // dbg!(&submesh.len());
            // dbg!(bucket_idx);
            //                   u8 * 4 = u32 and we have 3 of it
            let offset = (bucket_idx * (bucket_size * 4 * 3)) as u32;

            // dbg!(offset);
            self.vertex_buffer
                .set(offset, bytemuck::cast_slice(submesh))
        }
    }
}
