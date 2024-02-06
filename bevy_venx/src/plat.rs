use bevy::{ecs::event::Event, prelude::*};

pub enum MountPlatEv {
    New { depth: u8 },
}

pub enum Plat {
    Loaded {},
    LoadFromMCA {},
}
