use bevy::prelude::*;



pub struct BevyVenx;

impl Plugin for BevyVenx {
    fn build(&self, _app: &mut App) {
        // app.add_event::<MountPlatEv>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn manual_set() {}
}
