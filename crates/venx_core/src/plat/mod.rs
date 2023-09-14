use crate::{chunk::storage::ChunksStorage, controller::Controller};

pub struct Plat {
    controller: Controller,
    chunks: ChunksStorage,
}

impl Plat {
    pub fn load() {}
    pub fn save() {}
    pub fn load_mca() {}
}
