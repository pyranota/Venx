use bevy::prelude::*;

use crate::{
    plat_material::{setup_voxel_pool, CustomMaterialPlugin, PlatMaterialData},
    res::{BevyPlat, LoadPlat, LoadPlatOr, PlFocus},
};

pub struct BevyVenx;

impl Plugin for BevyVenx {
    fn build(&self, app: &mut App) {
        app.add_plugins(CustomMaterialPlugin)
            .add_event::<LoadPlat>()
            .add_event::<LoadPlatOr>()
            .add_systems(Startup, setup_voxel_pool)
            .add_systems(
                Update,
                (
                    // set_focus,
                    crate::task_runner::submit_tasks,
                    crate::task_runner::poll_tasks,
                ),
            );
    }
}

fn set_focus(focus_q: Query<&Transform, With<PlFocus>>, mut plat_q: Query<&mut BevyPlat>) {
    let focus = focus_q.single();
    for mut plat in &mut plat_q {
        plat.focus_set(focus.translation.to_array(), focus.rotation.to_array());
    }
}
