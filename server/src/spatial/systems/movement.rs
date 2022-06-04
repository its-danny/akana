use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::{
        events::NetworkMessage,
        server::{ConnectionId, NetworkServer},
    },
    player::components::{client::Client, online::Online},
    spatial::components::{collider::Collider, position::Position},
    world::components::tile::Tile,
};

/// Handles movement commands.
#[allow(clippy::type_complexity)]
pub fn movement(
    server: Res<NetworkServer>,
    mut messages: ParamSet<(EventReader<NetworkMessage>, EventWriter<NetworkMessage>)>,
    mut players: Query<(&Client, &mut Position), With<Online>>,
    tiles: Query<&Position, (With<Tile>, Without<Client>)>,
    colliders: Query<&Position, (With<Collider>, Without<Client>)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new(
            "^(north|n|northeast|ne|east|e|southeast|se|south|s|southwest|sw|west|w|northwest|nw)$"
        )
        .unwrap();
    }

    let mut moved: Vec<ConnectionId> = Vec::new();

    for message in messages.p0().iter() {
        if let Some((client, mut position)) = players.iter_mut().find(|p| p.0.id == message.id) {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                let wanted_tile = match captures.get(0).unwrap().as_str() {
                    "north" | "n" => tiles.iter().find(|p| p.0 == position.0 + IVec2::new(0, -1)),
                    "northeast" | "ne" => {
                        tiles.iter().find(|p| p.0 == position.0 + IVec2::new(1, -1))
                    }
                    "east" | "e" => tiles.iter().find(|p| p.0 == position.0 + IVec2::new(1, 0)),
                    "southeast" | "se" => {
                        tiles.iter().find(|p| p.0 == position.0 + IVec2::new(1, 1))
                    }
                    "south" | "s" => tiles.iter().find(|p| p.0 == position.0 + IVec2::new(0, 1)),
                    "southwest" | "sw" => {
                        tiles.iter().find(|p| p.0 == position.0 + IVec2::new(-1, 1))
                    }
                    "west" | "w" => tiles.iter().find(|p| p.0 == position.0 + IVec2::new(-1, 0)),
                    "northwest" | "nw" => tiles
                        .iter()
                        .find(|p| p.0 == position.0 + IVec2::new(-1, -1)),
                    _ => None,
                };

                if let Some(tile) = wanted_tile {
                    if colliders.iter().any(|c| c.0 == tile.0) {
                        server.send_message("Something blocks your way.", client.id);
                    } else {
                        position.0 = tile.0;

                        moved.push(client.id);
                    }
                } else {
                    server.send_message("You can't go that direction.", client.id);
                }
            }
        }
    }

    for id in moved {
        messages.p1().send({
            NetworkMessage {
                id,
                body: "look".to_string(),
                internal: true,
            }
        });
    }
}
