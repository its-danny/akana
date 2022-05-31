mod systems;

use bevy::prelude::*;

use self::systems::say;

pub(crate) struct SocialPlugin;

impl Plugin for SocialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(say);
    }
}
