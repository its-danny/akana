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
        spatial::components::collider::Collider,
        test::bundles::utils::{connection_id, door_bundle, open_door_bundle, player_bundle},
    };

    #[test]
    fn open() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Hunral", 0, 0));

        let door_id = app
            .world
            .spawn()
            .insert_bundle(door_bundle(true, 0, 1))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        assert!(app.world.get::<Collider>(door_id).is_none());

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "The door opens.");
    }

    #[test]
    fn already_open() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Hunral", 0, 0));

        app.world
            .spawn()
            .insert_bundle(open_door_bundle(true, 0, 1));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "It's already open!");
    }

    #[test]
    fn close() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Hunral", 0, 0));

        let door_id = app
            .world
            .spawn()
            .insert_bundle(open_door_bundle(true, 0, 1))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "close".into(),
                internal: false,
            });

        app.update();

        assert!(app.world.get::<Collider>(door_id).is_some());

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "The door closes.");
    }

    #[test]
    fn already_closed() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Hunral", 0, 0));

        app.world.spawn().insert_bundle(door_bundle(true, 0, 1));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "close".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "It's already closed!");
    }

    #[test]
    fn no_nearby_door() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::toggle_door);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Hunral", 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "open".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "There's no doors here!");
    }
}
