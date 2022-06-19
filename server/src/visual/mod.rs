use bevy::prelude::*;

pub mod components;
pub mod palette;

use self::palette::Palette;

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Palette::default());
    }
}
