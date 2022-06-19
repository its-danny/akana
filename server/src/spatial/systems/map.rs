use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::NetworkClient, online::Online},
    spatial::components::position::Position,
    visual::{
        components::sprite::{Sprite, SpritePaint},
        palette::{hex_to_rgb, rgb_to_color, Palette},
    },
};

/// Handles the `map` command.
pub fn map(
    palette: Res<Palette>,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Position, &Sprite), With<Online>>,
    sprites: Query<(&Position, &Sprite), Without<NetworkClient>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(map|m)$").unwrap();
    }

    for message in input.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some((client, position, player_sprite)) =
                players.iter().find(|(c, _, _)| c.id == message.id)
            {
                let map_width = client.width;
                let map_height = 16;

                let mut map = vec![
                    vec![Paint::new(" ").bg(palette.slate[9]); map_width as usize];
                    map_height as usize
                ];

                let start_x = position.0.x - (map_width as i32 / 2);
                let end_x = position.0.x + (map_width as i32 / 2);
                let start_y = position.0.y - (map_height as i32 / 2);
                let end_y = position.0.y + (map_height as i32 / 2);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        // We'll use the background color of the first entity on this
                        // space (usually the tile) in case the one below doesn't have one.
                        let first = sprites.iter().find(|s| s.0 .0 == IVec2::new(x, y));

                        // Since we're creating the entities layer-by-layer,
                        // the last one at a given position is which should be rendered.
                        if let Some(sprite) =
                            sprites.iter().filter(|s| s.0 .0 == IVec2::new(x, y)).last()
                        {
                            let mut sprite = if sprite.0 .0 == position.0 {
                                player_sprite.paint()
                            } else {
                                sprite.1.paint()
                            };

                            if let Some(background) = &player_sprite.background {
                                sprite = sprite.bg(rgb_to_color(hex_to_rgb(background)));
                            } else if let Some((_, s)) = first {
                                if let Some(background) = &s.background {
                                    sprite = sprite.bg(rgb_to_color(hex_to_rgb(background)));
                                }
                            }

                            map[(y - start_y - 1).clamp(0, map_height) as usize]
                                [(x - start_x - 1).clamp(0, map_width) as usize] = sprite;
                        }
                    }
                }

                let display = map
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|paint| paint.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                output.send(NetworkOutput {
                    id: client.id,
                    body: format!("{}\r\n", display),
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
        player::components::client::NetworkClient,
        test::bundles::utils::{player_bundle, tile_bundle, PlayerBundle, TileBundle},
        visual::palette::Palette,
    };

    #[test]
    fn map() {
        Paint::disable();

        let mut app = App::new();

        app.insert_resource(Palette::default());
        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::map);

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
            x: 1,
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
                body: "map".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body.matches(".").count(), 2);
        assert_eq!(output.body.matches("@").count(), 1);
    }
}
