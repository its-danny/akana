use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::Client, online::Online},
    spatial::components::position::Position,
    visual::components::sprite::{Sprite, SpritePaint},
    world::components::tile::Tile,
};

/// Handles the `look` command.
pub fn look(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&Client, &Position), With<Online>>,
    tiles: Query<(&Tile, &Position, &Sprite)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(look|l)$").unwrap();
    }

    for message in input.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some((client, position)) = players.iter().find(|(c, _)| c.id == message.id) {
                if let Some((tile, _, sprite)) = tiles.iter().find(|(_, p, _)| p.0 == position.0) {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: format!("{} {}\r\n{}", sprite.paint(), tile.name, tile.description),
                    });
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
        test::bundles::utils::{connection_id, player_bundle, tile_bundle},
    };

    #[test]
    fn look() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::look);

        let id = connection_id();
        app.world.spawn().insert_bundle(player_bundle(id, 0, 0));

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
}
