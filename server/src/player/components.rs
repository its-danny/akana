use bevy::prelude::*;

use crate::network::server::ConnectionId;

#[derive(Component)]
pub(crate) struct Client {
    pub(crate) id: ConnectionId,
    pub(crate) width: i32,
}

#[derive(Component)]
pub(crate) struct Account(pub(crate) i32);

#[derive(Component)]
pub(crate) struct Character {
    pub(crate) name: String,
}
