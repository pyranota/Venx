use bevy::prelude::*;
use venx::plat::VenxPlat;

pub enum MountPlatEv {
    New { depth: usize },
}

#[derive(Component)]
pub struct BevyPlat(pub VenxPlat);
