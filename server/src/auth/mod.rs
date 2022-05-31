mod api;
pub(crate) mod components;
mod systems;

use bevy::prelude::*;

use systems::{perform_authentication, start_authenticating_new_clients};

pub(crate) struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("auth")
                .with_system(start_authenticating_new_clients)
                .with_system(perform_authentication),
        );
    }
}
