use bevy::prelude::*;

use crate::network::server::ConnectionId;

#[derive(Component)]
pub struct NetworkClient {
    pub id: ConnectionId,
    pub width: i32,
}
