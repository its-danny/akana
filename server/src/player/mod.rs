pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use self::systems::handle_network_events;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_network_events);
    }
}
