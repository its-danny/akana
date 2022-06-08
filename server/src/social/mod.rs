mod systems;

use bevy::prelude::*;

use self::systems::{emote::*, say::*};

pub struct SocialPlugin;

impl Plugin for SocialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("social")
                .with_system(say)
                .with_system(emote),
        );
    }
}
