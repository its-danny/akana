use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    auth::components::Online,
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::Client,
    visual::components::{Sprite, SpritePaint},
    world::components::Tile,
};

use super::components::{Collider, Door, Position};

/// Handles the `look` command.
pub(crate) fn look(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<((&Client, &Position), With<Online>)>,
    tiles: Query<(&Tile, &Position, &Sprite)>,
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
                            tile.2.paint(),
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
    players: Query<(&Client, &Position), With<Online>>,
    sprites: Query<(&Position, &Sprite)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(map|m)$").unwrap();
    }

    for message in messages.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some(player) = players.iter().find(|p| p.0.id == message.id) {
                let position = player.1 .0;

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
                        // Since we're creating the entities layer-by-layer,
                        // the last one at a given position is which should be rendered.
                        if let Some(sprite) = sprites
                            .iter()
                            .filter(|s| s.0 .0 == IVec3::new(x, y, 0))
                            .last()
                        {
                            let sprite = if sprite.0 .0 == position {
                                Paint::white("@").bold()
                            } else {
                                sprite.1.paint()
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
#[allow(clippy::type_complexity)]
pub(crate) fn movement(
    server: Res<NetworkServer>,
    mut message_reader: EventReader<NetworkMessage>,
    mut players: Query<(&Client, &mut Position), With<Online>>,
    tiles: Query<(&Tile, &Position), Without<Client>>,
    colliders: Query<&Position, (With<Collider>, Without<Client>, Without<Tile>)>,
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
                    if colliders.iter().any(|c| c.0 == tile.1 .0) {
                        server.send("Something blocks your way.", player.0.id);
                    } else {
                        debug!("Moving {:?} to {:?}", player.0.id, tile.1);

                        player.1 .0 = tile.1 .0;
                    }
                } else {
                    server.send("You can't go that direction.", player.0.id);
                }
            }
        }
    }
}

/// Handles opening and closing doors
#[allow(clippy::type_complexity)]
pub(crate) fn manage_doors(
    mut commands: Commands,
    server: Res<NetworkServer>,
    mut message_reader: EventReader<NetworkMessage>,
    players: Query<(&Client, &mut Position), With<Online>>,
    mut doors: Query<
        (Entity, &mut Door, &Position, &mut Sprite, Option<&Collider>),
        Without<Client>,
    >,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(open|close)$").unwrap();
    }

    for message in message_reader.iter() {
        if let Some(player) = players.iter().find(|p| p.0.id == message.id) {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if let Some(mut door) = doors.iter_mut().find(|d| {
                    d.2 .0 == player.1 .0 + IVec3::new(0, 1, 0)
                        || d.2 .0 == player.1 .0 + IVec3::new(0, -1, 0)
                        || d.2 .0 == player.1 .0 + IVec3::new(1, 0, 0)
                        || d.2 .0 == player.1 .0 + IVec3::new(-1, 0, 0)
                }) {
                    match captures.get(0).unwrap().as_str() {
                        "open" => {
                            match door.4 {
                                Some(_) => {
                                    door.3.character = "/".to_string();
                                    commands.entity(door.0).remove::<Collider>();
                                    server.send("The door opens.", player.0.id);
                                }
                                None => server.send("It's already open!", player.0.id),
                            };
                        }
                        "close" => match door.4 {
                            None => {
                                door.3.character = if door.1.facing == "horizontal" {
                                    "|"
                                } else {
                                    "-"
                                }
                                .to_string();

                                commands.entity(door.0).insert(Collider);
                                server.send("The door closes.", player.0.id);
                            }
                            Some(_) => server.send("It's already closed!", player.0.id),
                        },
                        _ => {}
                    }
                } else {
                    server.send("There's nothing to open over here!", player.0.id);
                }
            }
        }
    }
}
