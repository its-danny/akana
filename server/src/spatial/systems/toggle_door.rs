use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::NetworkClient, online::Online},
    spatial::components::{collider::Collider, door::Door, position::Position},
    visual::components::sprite::Sprite,
};

/// Handles opening and closing doors
pub fn toggle_door(
    mut commands: Commands,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &mut Position), With<Online>>,
    mut doors: Query<
        (Entity, &Door, &Position, &mut Sprite, Option<&Collider>),
        Without<NetworkClient>,
    >,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(open|close)$").unwrap();
    }

    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
            if let Some((client, position)) = players.iter().find(|p| p.0.id == message.id) {
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

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};

    use crate::{
        network::events::{NetworkInput, NetworkOutput},
        player::components::client::NetworkClient,
        spatial::components::collider::Collider,
        test::bundles::utils::{
            closed_door_bundle, open_door_bundle, player_bundle, DoorBundle, PlayerBundle,
        },
    };

    #[test]
    fn open() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        let door = app
            .world
            .spawn()
            .insert_bundle(closed_door_bundle(DoorBundle {
                y: 1,
                ..Default::default()
            }))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        assert!(app.world.get::<Collider>(door).is_none());

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "The door opens.");
    }

    #[test]
    fn already_open() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .spawn()
            .insert_bundle(open_door_bundle(DoorBundle {
                y: 1,
                ..Default::default()
            }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "It's already open!");
    }

    #[test]
    fn close() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                y: 1,
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        let door = app
            .world
            .spawn()
            .insert_bundle(open_door_bundle(DoorBundle {
                ..Default::default()
            }))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "close".into(),
                internal: false,
            });

        app.update();

        assert!(app.world.get::<Collider>(door).is_some());

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "The door closes.");
    }

    #[test]
    fn already_closed() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                y: 1,
                ..Default::default()
            }))
            .insert(Collider)
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .spawn()
            .insert_bundle(closed_door_bundle(DoorBundle {
                ..Default::default()
            }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "close".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "It's already closed!");
    }

    #[test]
    fn no_nearby_door() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "There's no doors here!");
    }
}
