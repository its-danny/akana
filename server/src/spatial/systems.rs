use bevy::prelude::*;

use crate::{
    auth::components::Online,
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::Client,
    world::components::Tile,
};

use super::components::Position;

/// Handles the `look` command
pub(crate) fn look(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<((&Client, &Position), With<Online>)>,
    tiles: Query<(&Tile, &Position)>,
) {
    for message in messages.iter() {
        if message.body == "look" {
            if let Some(player) = players.iter().find(|p| p.0 .0 .0 == message.id) {
                if let Some(tile) = tiles.iter().find(|t| t.1 .0 == player.0 .1 .0) {
                    server.send(&tile.0.name, message.id);
                }
            }
        }
    }
}
