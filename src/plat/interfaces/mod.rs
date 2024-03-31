// pub mod canvas;
pub mod load;
// pub mod image;
pub mod layer;

use downcast_rs::Downcast;

use self::{layer::LayerInterface, load::LoadInterface};

pub trait PlatInterface: LayerInterface + LoadInterface + Downcast {}
