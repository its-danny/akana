use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::Client, online::Online},
    spatial::components::{position::Position, tile::Tile},
    visual::components::{
        details::Details,
        sprite::{Sprite, SpritePaint},
    },
};

/// Handles the `look` command.
pub fn look(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&Client, &Position), With<Online>>,
    entities: Query<(&Position, &Details, &Sprite), Without<Tile>>,
    tiles: Query<(&Position, &Details, &Sprite), With<Tile>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(look|l)( )?(.+)?$").unwrap();
    }

    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
            if let Some((client, position)) = players.iter().find(|(c, _)| c.id == message.id) {
                match captures.get(3) {
                    // Look at a specific entity by name.
                    Some(name) => {
                        match entities.iter().find(|(p, d, _)| {
                            p.0 == position.0
                                && d.name.to_lowercase() == name.as_str().to_lowercase().trim()
                        }) {
                            Some((_, details, sprite)) => {
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
                    // If no name is provided, look at the player's current tile.
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
        test::bundles::utils::{connection_id, door_bundle, player_bundle, tile_bundle},
    };

    #[test]
    fn look() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Rodrani", 0, 0));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room", "Please ignore.", 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "look".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(&output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert!(output.body.contains(&Paint::black(".").bold().to_string()));
        assert!(output.body.contains("Test Room"));
        assert!(output.body.contains("Please ignore."));
    }

    #[test]
    fn at_entity() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Rodrani", 0, 0));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room", "Please ignore.", 0, 0));

        app.world.spawn().insert_bundle(door_bundle(false, 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "look door".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(&output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert!(output.body.contains(&Paint::white("-").to_string()));
        assert!(output.body.contains("Door"));
        assert!(output.body.contains("A door."));
    }

    #[test]
    fn entity_not_found() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Rodrani", 0, 0));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room", "Please ignore.", 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "look door".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(&output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body, "You don't see that here.".to_string());
    }
}
