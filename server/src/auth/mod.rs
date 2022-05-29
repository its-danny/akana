pub(crate) mod components;
mod systems;
mod utils;

use bevy::prelude::*;

use systems::{handle_network_messages, start_authenticating};

pub(crate) struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_authenticating);
        app.add_system(handle_network_messages);
    }
}
