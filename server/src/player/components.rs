use bevy::prelude::*;

use crate::network::server::ConnectionId;

#[derive(Component)]
pub(crate) struct Client(pub(crate) ConnectionId);
