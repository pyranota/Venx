use crate::voxel::interfaces::chunk_loader::ChunkLoaderInterface;

use self::canvas::Canvas;
use self::image::Image;

pub mod canvas;
pub mod image;

#[derive(Debug)]
pub enum Layer {
    Image(Image),
    Canvas(Canvas),
}
