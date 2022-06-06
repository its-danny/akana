pub mod components;
mod systems;

use bevy::prelude::*;

use self::systems::{look::*, map::*, movement::*, peer::*, toggle_door::*};

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("spatial")
                .with_system(look)
                .with_system(peer)
                .with_system(map)
                .with_system(movement)
                .with_system(toggle_door),
        );
    }
}
