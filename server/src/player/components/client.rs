use bevy::prelude::*;

use crate::network::server::ConnectionId;

#[derive(Component)]
pub struct Client {
    pub id: ConnectionId,
    pub width: i32,
}
