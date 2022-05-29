use bevy::prelude::*;

use crate::network::server::ConnectionId;

#[derive(Component)]
pub(crate) struct Player(pub(crate) ConnectionId);
