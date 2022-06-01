use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    auth::components::Online,
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::Client,
    world::components::{Tile, TilePaint},
};

use super::components::Position;

/// Handles the `look` command.
pub(crate) fn look(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<((&Client, &Position), With<Online>)>,
    tiles: Query<(&Tile, &Position)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(look|l)$").unwrap();
    }

    for message in messages.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some(player) = players.iter().find(|p| p.0 .0.id == message.id) {
                if let Some(tile) = tiles.iter().find(|t| t.1 .0 == player.0 .1 .0) {
                    server.send(
                        &format!(
                            "{} {}\r\n{}",
                            tile.0.paint(),
                            tile.0.name,
                            tile.0.description
                        ),
                        message.id,
                    );
                }
            }
        }
    }
}

/// Handles the `map` command.
pub(crate) fn map(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<((&Client, &Position), With<Online>)>,
    tiles: Query<(&Tile, &Position)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(map|m)$").unwrap();
    }

    for message in messages.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some(player) = players.iter().find(|p| p.0 .0.id == message.id) {
                let position = player.0 .1 .0;

                const MAP_WIDTH: i32 = 80;
                const MAP_HEIGHT: i32 = 20;

                let mut map: [[Paint<&str>; MAP_WIDTH as usize]; MAP_HEIGHT as usize] =
                    [[Paint::new(" "); MAP_WIDTH as usize]; MAP_HEIGHT as usize];

                let start_x = position.x - (MAP_WIDTH as i32 / 2);
                let end_x = position.x + (MAP_WIDTH as i32 / 2);
                let start_y = position.y - (MAP_HEIGHT as i32 / 2);
                let end_y = position.y + (MAP_HEIGHT as i32 / 2);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        if let Some(tile) = tiles.iter().find(|t| t.1 .0 == IVec3::new(x, y, 0)) {
                            let sprite = if tile.1 .0 == position {
                                Paint::white("@").bold()
                            } else {
                                tile.0.paint()
                            };

                            map[(y - start_y - 1).clamp(0, MAP_HEIGHT) as usize]
                                [(x - start_x - 1).clamp(0, MAP_WIDTH) as usize] = sprite;
                        }
                    }
                }

                server.send(
                    &map.map(|r| r.map(|c| format!("{}", c)).join(""))
                        .join("\r\n"),
                    message.id,
                );
            }
        }
    }
}

/// Handles movement commands.
pub(crate) fn movement(
    server: Res<NetworkServer>,
    mut message_reader: EventReader<NetworkMessage>,
    mut players: Query<(&Client, &mut Position), With<Online>>,
    tiles: Query<(&Tile, &Position), Without<Client>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new(
            "^(north|n|northeast|ne|east|e|southeast|se|south|s|southwest|sw|west|w|northwest|nw)$"
        )
        .unwrap();
    }

    for message in message_reader.iter() {
        if let Some(mut player) = players.iter_mut().find(|p| p.0.id == message.id) {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                let wanted_tile = match captures.get(0).unwrap().as_str() {
                    "north" | "n" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(0, -1, 0)),
                    "northeast" | "ne" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, -1, 0)),
                    "east" | "e" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, 0, 0)),
                    "southeast" | "se" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(1, 1, 0)),
                    "south" | "s" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(0, 1, 0)),
                    "southwest" | "sw" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, 1, 0)),
                    "west" | "w" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, 0, 0)),
                    "northwest" | "nw" => tiles
                        .iter()
                        .find(|t| t.1 .0 == player.1 .0 + IVec3::new(-1, -1, 0)),
                    _ => None,
                };

                if let Some(tile) = wanted_tile {
                    debug!("Moving {:?} to {:?}", player.0.id, tile.1);

                    player.1 .0 = tile.1 .0;
                } else {
                    server.send("You can't go that direction.", player.0.id);
                }
            }
        }
    }
}
