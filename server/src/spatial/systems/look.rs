use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::NetworkClient, online::Online},
    spatial::components::{position::Position, tile::Tile},
    visual::components::{
        details::Details,
        sprite::{Sprite, SpritePaint},
    },
};

/// Send a description of the tile the player is currently on or
/// an entity if they target one.
pub fn look(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Position), With<Online>>,
    entities: Query<(Entity, &Position, &Details, &Sprite), Without<Tile>>,
    tiles: Query<(&Position, &Details, &Sprite), With<Tile>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(look|l)( )?(.+)?$").unwrap();
    }

    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
            if let Some((client, position)) = players.iter().find(|(c, _)| c.id == message.id) {
                match captures.get(3) {
                    // Look at a specific entity by name or ID in the same tile
                    // as the player.
                    Some(name_or_id) => {
                        match entities.iter().find(|(e, p, d, _)| {
                            p.0 == position.0
                                && (d.name.to_lowercase()
                                    == name_or_id.as_str().to_lowercase().trim()
                                    || e.id().to_string() == name_or_id.as_str())
                        }) {
                            Some((_, _, details, sprite)) => {
                                output.send(NetworkOutput {
                                    id: client.id,
                                    body: format!(
                                        "{} {}\r\n{}",
                                        sprite.paint(),
                                        details.name,
                                        details.description
                                    ),
                                });
                            }
                            None => {
                                output.send(NetworkOutput {
                                    id: client.id,
                                    body: "You don't see that here.".into(),
                                });
                            }
                        }
                    }
                    // If none provided, look at tile.
                    None => {
                        if let Some((_, details, sprite)) =
                            tiles.iter().find(|(p, _, _)| p.0 == position.0)
                        {
                            output.send(NetworkOutput {
                                id: client.id,
                                body: format!(
                                    "{} {}\r\n{}",
                                    sprite.paint(),
                                    details.name,
                                    details.description
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};
    use yansi::Paint;

    use crate::{
        network::events::{NetworkInput, NetworkOutput},
        player::components::client::NetworkClient,
        test::bundles::utils::{
            closed_door_bundle, open_door_bundle, player_bundle, tile_bundle, DoorBundle,
            PlayerBundle, TileBundle,
        },
        visual::components::{details::Details, sprite::Sprite},
    };

    #[test]
    fn look() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            name: "Test Room".into(),
            description: "Please ignore.".into(),
            ..Default::default()
        }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "look".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, ". Test Room\r\nPlease ignore.");
    }

    #[test]
    fn at_entity_by_name() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            ..Default::default()
        }));

        let door = app
            .world
            .spawn()
            .insert_bundle(open_door_bundle(DoorBundle {
                name: "Front Door".into(),
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: format!("look front door"),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);

        assert_eq!(
            output.body,
            format!(
                "{} {}\r\n{}",
                app.world.get::<Sprite>(door).unwrap().character,
                app.world.get::<Details>(door).unwrap().name,
                app.world.get::<Details>(door).unwrap().description,
            )
        );
    }

    #[test]
    fn at_entity_by_id() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            ..Default::default()
        }));

        let door = app
            .world
            .spawn()
            .insert_bundle(closed_door_bundle(DoorBundle {
                ..Default::default()
            }))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: format!("look {}", door.id().to_string()),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);

        assert_eq!(
            output.body,
            format!(
                "{} {}\r\n{}",
                app.world.get::<Sprite>(door).unwrap().character,
                app.world.get::<Details>(door).unwrap().name,
                app.world.get::<Details>(door).unwrap().description,
            )
        );
    }

    #[test]
    fn entity_not_found() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            ..Default::default()
        }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "look door".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You don't see that here.".to_string());
    }
}
