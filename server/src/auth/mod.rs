pub mod components;
mod systems;
mod utils;

use bevy::prelude::*;

use self::systems::perform_authentication::*;

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("auth")
                .with_system(perform_authentication),
        );
    }
}
