use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::Client, online::Online},
    spatial::components::position::Position,
    visual::components::sprite::{Sprite, SpritePaint},
};

/// Handles the `map` command.
pub fn map(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&Client, &Position), With<Online>>,
    sprites: Query<(&Position, &Sprite)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(map|m)$").unwrap();
    }

    for message in input.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some((client, position)) = players.iter().find(|(c, _)| c.id == message.id) {
                const MAP_WIDTH: i32 = 80;
                const MAP_HEIGHT: i32 = 10;

                let mut map: [[Paint<&str>; MAP_WIDTH as usize]; MAP_HEIGHT as usize] =
                    [[Paint::new(" "); MAP_WIDTH as usize]; MAP_HEIGHT as usize];

                let start_x = position.0.x - (MAP_WIDTH as i32 / 2);
                let end_x = position.0.x + (MAP_WIDTH as i32 / 2);
                let start_y = position.0.y - (MAP_HEIGHT as i32 / 2);
                let end_y = position.0.y + (MAP_HEIGHT as i32 / 2);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        // Since we're creating the entities layer-by-layer,
                        // the last one at a given position is which should be rendered.
                        if let Some(sprite) =
                            sprites.iter().filter(|s| s.0 .0 == IVec2::new(x, y)).last()
                        {
                            let sprite = if sprite.0 .0 == position.0 {
                                Paint::white("@").bold()
                            } else {
                                sprite.1.paint()
                            };

                            map[(y - start_y - 1).clamp(0, MAP_HEIGHT) as usize]
                                [(x - start_x - 1).clamp(0, MAP_WIDTH) as usize] = sprite;
                        }
                    }
                }

                output.send(NetworkOutput {
                    id: client.id,
                    body: map
                        .map(|r| r.map(|c| format!("{}", c)).join(""))
                        .join("\r\n"),
                });
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
    fn map() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::map);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Mussugro", 0, 0));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room 1", "Please ignore.", 0, 0));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room 2", "Please ignore.", 0, 1));

        app.world
            .spawn()
            .insert_bundle(tile_bundle("Test Room 2", "Please ignore.", 1, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
                body: "map".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, id);
        assert_eq!(output.body.matches(".").count(), 2);
        assert_eq!(output.body.matches("@").count(), 1);
    }
}
