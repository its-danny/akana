pub(crate) mod components;
mod systems;

use bevy::prelude::*;

use self::systems::look;

pub(crate) struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(look);
    }
}
