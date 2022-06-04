use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::{client::Client, online::Online},
    spatial::components::position::Position,
    visual::components::sprite::{Sprite, SpritePaint},
};

/// Handles the `map` command.
pub fn map(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<(&Client, &Position), With<Online>>,
    sprites: Query<(&Position, &Sprite)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(map|m)$").unwrap();
    }

    for message in messages.iter() {
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

                server.send(
                    &map.map(|r| r.map(|c| format!("{}", c)).join(""))
                        .join("\r\n"),
                    client.id,
                );
            }
        }
    }
}
