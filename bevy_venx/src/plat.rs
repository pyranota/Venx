use bevy::{ecs::event::Event, prelude::*};

pub enum MountPlatEv {
    New { depth: usize },
}

pub enum Plat {
    Loaded {},
    LoadFromMCA {},
}
