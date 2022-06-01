use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct Position(pub(crate) IVec3);

#[derive(Component)]
pub(crate) struct Collider;
