use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::{client::Client, online::Online},
    spatial::components::{collider::Collider, door::Door, position::Position},
    visual::components::sprite::Sprite,
};

/// Handles opening and closing doors
#[allow(clippy::type_complexity)]
pub fn toggle_door(
    mut commands: Commands,
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<(&Client, &mut Position), With<Online>>,
    mut doors: Query<(Entity, &Door, &Position, &mut Sprite, Option<&Collider>), Without<Client>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(open|close)$").unwrap();
    }

    for message in messages.iter() {
        if let Some((client, position)) = players.iter().find(|p| p.0.id == message.id) {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if let Some((entity, door, _, mut sprite, collider)) =
                    doors.iter_mut().find(|(_, _, p, _, _)| {
                        p.0 == position.0 + IVec2::new(0, 1)
                            || p.0 == position.0 + IVec2::new(0, -1)
                            || p.0 == position.0 + IVec2::new(1, 0)
                            || p.0 == position.0 + IVec2::new(-1, 0)
                    })
                {
                    match captures.get(0).unwrap().as_str() {
                        "open" => {
                            match collider {
                                Some(_) => {
                                    sprite.character = "/".to_string();
                                    commands.entity(entity).remove::<Collider>();
                                    server.send("The door opens.", client.id);
                                }
                                None => server.send("It's already open!", client.id),
                            };
                        }
                        "close" => match collider {
                            None => {
                                sprite.character = if door.facing == "horizontal" {
                                    "|"
                                } else {
                                    "-"
                                }
                                .to_string();

                                commands.entity(entity).insert(Collider);
                                server.send("The door closes.", client.id);
                            }
                            Some(_) => server.send("It's already closed!", client.id),
                        },
                        _ => {}
                    }
                } else {
                    server.send("There's no doors here!", client.id);
                }
            }
        }
    }
}
