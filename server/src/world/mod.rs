pub mod components;
mod systems;

use bevy::prelude::*;

use self::systems::setup_world;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_world);
    }
}
