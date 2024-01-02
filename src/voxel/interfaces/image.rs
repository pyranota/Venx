use super::chunk_loader::ChunkLoaderInterface;

pub trait ImageInterface: ChunkLoaderInterface {
    fn insert_segment();
    // fn merge_in(image_handle)
}
