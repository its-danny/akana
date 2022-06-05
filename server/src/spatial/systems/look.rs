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
