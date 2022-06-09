use bevy::prelude::*;

use self::systems::{backpack::*, drop::*, take::*};

pub mod components;
pub mod systems;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("items")
                .with_system(take)
                .with_system(drop)
                .with_system(backpack),
        );
    }
}
