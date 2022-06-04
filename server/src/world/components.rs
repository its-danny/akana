use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Tile {
    pub(crate) name: String,
    pub(crate) description: String,
}
