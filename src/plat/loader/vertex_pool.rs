use anyhow::bail;

use super::{external_buffer::ExternalBuffer, BucketIdx};

pub type ExternalBufferObject = Box<dyn ExternalBuffer + 'static + Send + Sync>;

pub struct VertexPool {
    // TODO: Use linked list?
    pub free_buckets: Vec<BucketIdx>,
    /// Amount of faces (one face = 6 vertices) in single bucket
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
    pub(super) fn new(
        bucket_size: u32,
        bucket_amount: u32,
        indirect_buffer: ExternalBufferObject,
        vertex_buffer: ExternalBufferObject,
    ) -> Self {
        let bucket_usage = vec![500, 1000, 5000];
        let bucket_size = 256;
        let bucket_amount = 6500;

        assert_eq!(bucket_amount, bucket_usage.iter().sum::<u32>());

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
    pub(super) fn allocate(&mut self, amount: u32) -> anyhow::Result<Vec<BucketIdx>> {
        let buckets = &mut self.free_buckets;
        if buckets.len() as u32 >= amount {
            bail!("You ran out of free buckets, cannot allocate anymore. \n Consider deallocating some before trying again");
        }
        Ok(buckets.split_off(buckets.len() - amount as usize))
    }

    pub(super) fn deallocate(&mut self, mut buckets: Vec<BucketIdx>) {
        self.free_buckets.append(&mut buckets);
    }
}
