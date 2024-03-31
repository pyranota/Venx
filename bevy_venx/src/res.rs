use bevy::{
    ecs::{component::Component, event::Event, system::Resource},
    math::UVec3,
    prelude::{Deref, DerefMut},
    render::render_resource::Buffer,
    tasks::Task,
};
use venx::plat::VenxPlat;

#[derive(Event)]
/// Loads plat from fs with given path
pub struct LoadPlat<'a>(pub &'a str);

#[derive(Event)]
/// Tries to load plat, if no plat found by given path or plat is corrupted or has some other issues,
/// It will allow you to create new plat.
///
/// ```
/// ev_writer.emit(LoadPlatOr{
///     path: "path/to/your/plat",
///     // Save to "path/to/your/plat"
///     save_if_not_found: true,
///     or: |e| {
///         VenxPlat::convert_vox("path/to/your/magical/voxel/file");
///     }
/// })
/// ```
pub struct LoadPlatOr<'a> {
    pub path: &'a str,
    /// Save to fs by given path if `or` was called
    pub save_if_not_found: bool,
    pub or: fn() -> VenxPlat,
}

#[derive(Event)]
/// Indicates when plat is loaded and can be accessed by querying it.
pub struct PlatLoaded;

#[derive(Component, Deref, DerefMut)]
/// Gives ability to directly interact with plat
///
/// Be aware, if you want to perform heavy computations, you need to use bevy tasks. Otherwise you will introduce possible stutters.
pub struct BevyPlat(pub VenxPlat);

#[derive(Component, Deref, DerefMut)]
pub struct VenxTasks(pub Vec<Task<()>>);

#[derive(Component)]
/// Mark active entity (e.g. camera), marked entity should have [bevy::prelude::Transform] component.
///
/// So Venx voxel engine can adjust LoD's and loaded chunks in general
///
/// Having multiple Focuses is not supported yet
pub struct PlFocus;
