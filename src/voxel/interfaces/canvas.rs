use glam::UVec3;

use crate::voxel::segment::Segment;

use super::chunk_loader::ChunkLoaderInterface;

pub trait CanvasInterface: ChunkLoaderInterface {
    fn finalize_into_image();
    fn insert();
    fn insert_segment(&mut self, segment: Segment, position: UVec3);
    fn remove();
}
