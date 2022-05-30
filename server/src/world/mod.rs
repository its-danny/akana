pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use self::{resources::NewPlayerSpawn, systems::setup_world};

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NewPlayerSpawn::default());
        app.add_startup_system(setup_world);
    }
}
