use bevy::prelude::*;

use crate::plat::MountPlatEv;

pub struct BevyVenx;

impl Plugin for BevyVenx {
    fn build(&self, app: &mut App) {
        app.add_event::<MountPlatEv>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn manual_set() {}
}
