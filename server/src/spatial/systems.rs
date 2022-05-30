use bevy::prelude::*;

use crate::{
    auth::components::Online,
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::Client,
    world::components::Tile,
};

use super::components::Position;

/// Handles the `look` command.
pub(crate) fn look(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<((&Client, &Position), With<Online>)>,
    tiles: Query<(&Tile, &Position)>,
) {
    for message in messages.iter() {
        if message.body.to_lowercase() == "look" {
            if let Some(player) = players.iter().find(|p| p.0 .0 .0 == message.id) {
                if let Some(tile) = tiles.iter().find(|t| t.1 .0 == player.0 .1 .0) {
                    server.send(&tile.0.name, message.id);
                }
            }
        }
    }
}

/// Handles movement commands.
pub(crate) fn movement(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    mut players: Query<(&Client, &mut Position), With<Online>>,
    tiles: Query<(&Tile, &Position), Without<Client>>,
) {
    for message in messages.iter() {
        if let Some(mut player) = players.iter_mut().find(|p| p.0 .0 == message.id) {
            let result_if_direction = match message.body.to_lowercase().as_str() {
                "north" | "n" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(0, -1, 0)),
                ),
                "northeast" | "ne" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, -1, 0)),
                ),
                "east" | "e" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, 0, 0)),
                ),
                "southeast" | "se" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, 1, 0)),
                ),
                "south" | "s" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(0, 1, 0)),
                ),
                "southwest" | "sw" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, 1, 0)),
                ),
                "west" | "w" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, 0, 0)),
                ),
                "northwest" | "nw" => Some(
                    tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, -1, 0)),
                ),
                _ => None,
            };

            // This code is pretty bad, but I couldn't think of a better way
            // to do it.
            //
            // `result_if_direction` is Some if they sent a directional command,
            // `None` if not.
            //
            // `found_tile` is Some if we find a tile in the direction they want to go,
            // `None` if not.
            if let Some(found_tile) = result_if_direction {
                if let Some(tile) = found_tile {
                    debug!("Moving {:?} to {:?}", player.0 .0, tile.1);

                    player.1 .0 = tile.1 .0;
                } else {
                    server.send("You can't go that direction.", player.0 .0);
                }
            }
        }
    }
}
