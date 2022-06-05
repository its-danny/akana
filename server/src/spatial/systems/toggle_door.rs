use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::Client, online::Online},
    spatial::components::{collider::Collider, door::Door, position::Position},
    visual::components::sprite::Sprite,
};

/// Handles opening and closing doors
#[allow(clippy::type_complexity)]
pub fn toggle_door(
    mut commands: Commands,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&Client, &mut Position), With<Online>>,
    mut doors: Query<(Entity, &Door, &Position, &mut Sprite, Option<&Collider>), Without<Client>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(open|close)$").unwrap();
    }

    for message in input.iter() {
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
                                    sprite.character = door.opened_character.clone();
                                    commands.entity(entity).remove::<Collider>();

                                    output.send(NetworkOutput {
                                        id: client.id,
                                        body: "The door opens.".to_string(),
                                    });
                                }
                                None => output.send(NetworkOutput {
                                    id: client.id,
                                    body: "It's already open!".to_string(),
                                }),
                            };
                        }
                        "close" => match collider {
                            None => {
                                sprite.character = door.closed_character.clone();
                                commands.entity(entity).insert(Collider);

                                output.send(NetworkOutput {
                                    id: client.id,
                                    body: "The door closes.".to_string(),
                                });
                            }
                            Some(_) => output.send(NetworkOutput {
                                id: client.id,
                                body: "It's already closed!".to_string(),
                            }),
                        },
                        _ => {}
                    }
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "There's no doors here!".to_string(),
                    });
                }
            }
        }
    }
}
