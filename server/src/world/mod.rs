pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use self::{
    resources::{new_player_spawn::NewPlayerSpawn, world_time::WorldTime},
    systems::{setup_world::*, update_world_time::*},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NewPlayerSpawn::default());
        app.insert_resource(WorldTime::default());

        app.add_startup_system(setup_world);

        app.add_system_set(
            SystemSet::new()
                .label("world")
                .with_system(update_world_time),
        );
    }
}
