use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::{
        events::{NetworkInput, NetworkOutput},
        server::ConnectionId,
    },
    player::components::{client::NetworkClient, online::Online},
    spatial::components::{collider::Collider, position::Position, tile::Tile},
};

/// Handles movement commands.
pub fn movement(
    mut input: ParamSet<(EventReader<NetworkInput>, EventWriter<NetworkInput>)>,
    mut output: EventWriter<NetworkOutput>,
    mut players: Query<(&NetworkClient, &mut Position), With<Online>>,
    tiles: Query<&Position, (With<Tile>, Without<NetworkClient>)>,
    colliders: Query<&Position, (With<Collider>, Without<NetworkClient>)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new(
            "^(north|n|northeast|ne|east|e|southeast|se|south|s|southwest|sw|west|w|northwest|nw)$"
        )
        .unwrap();
    }

    let mut moved: Vec<ConnectionId> = Vec::new();

    for message in input.p0().iter() {
        if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
            if let Some((client, mut position)) = players.iter_mut().find(|p| p.0.id == message.id)
            {
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
                        output.send(NetworkOutput {
                            id: client.id,
                            body: "Something blocks your way.".to_string(),
                        });
                    } else {
                        position.0 = tile.0;

                        moved.push(client.id);
                    }
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "You can't go that direction.".to_string(),
                    });
                }
            }
        }
    }

    for id in moved {
        input.p1().send({
            NetworkInput {
                id,
                body: "look".to_string(),
                internal: true,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};

    use crate::{
        network::events::{NetworkInput, NetworkOutput},
        player::components::client::NetworkClient,
        spatial::components::{collider::Collider, position::Position},
        test::bundles::utils::{player_bundle, tile_bundle, PlayerBundle, TileBundle},
    };

    #[test]
    fn movement() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::movement);

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

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            y: 1,
            ..Default::default()
        }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "south".into(),
                internal: false,
            });

        app.update();

        assert_eq!(
            app.world.get::<Position>(player).unwrap().0,
            IVec2::new(0, 1)
        );
    }

    #[test]
    fn blocked() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::movement);

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
            .spawn()
            .insert_bundle(tile_bundle(TileBundle {
                y: 1,
                ..Default::default()
            }))
            .insert(Collider);

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "south".into(),
                internal: false,
            });

        app.update();

        assert_eq!(
            app.world.get::<Position>(player).unwrap().0,
            IVec2::new(0, 0)
        );

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "Something blocks your way.");
    }

    #[test]
    fn invalid() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::movement);

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
                body: "south".into(),
                internal: false,
            });

        app.update();

        assert_eq!(
            app.world.get::<Position>(player).unwrap().0,
            IVec2::new(0, 0)
        );

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You can't go that direction.");
    }

    #[test]
    fn look_sent_after() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::movement);

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

        app.world.spawn().insert_bundle(tile_bundle(TileBundle {
            y: 1,
            ..Default::default()
        }));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "south".into(),
                internal: false,
            });

        app.update();

        let input_events = app.world.resource::<Events<NetworkInput>>();
        let mut input_reader = input_events.get_reader();
        let input = input_reader.iter(&input_events).last().unwrap();

        assert_eq!(input.id, player_client_id);
        assert_eq!(input.body, "look");
        assert_eq!(input.internal, true);
    }
}
