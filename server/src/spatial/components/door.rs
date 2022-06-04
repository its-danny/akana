use bevy::prelude::*;

#[derive(Component)]
pub struct Door {
    pub opened_character: String,
    pub closed_character: String,
}
