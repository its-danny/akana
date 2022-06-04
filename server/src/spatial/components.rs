use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct Position(pub(crate) IVec3);

#[derive(Component)]
pub(crate) struct Collider;

#[derive(Component)]
pub(crate) struct Door {
    pub(crate) facing: String,
}
