use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{character::Character, client::NetworkClient, online::Online},
    spatial::components::{position::Position, tile::Tile},
    visual::{components::details::Details, palette::Palette},
};

/// Lists all entities in a room, excluding tiles, with their entity ID.
pub fn peer(
    palette: Res<Palette>,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Position, &Character), With<Online>>,
    entities: Query<(Entity, &Position, &Details), Without<Tile>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(peer|p)$").unwrap();
    }

    for message in input.iter() {
        if CMD.is_match(&message.body.to_lowercase()) {
            if let Some((client, position, _)) = players.iter().find(|(c, _, _)| c.id == message.id)
            {
                let ids = entities
                    .iter()
                    .filter(|(_, p, _)| p.0 == position.0)
                    .map(|(e, _, d)| {
                        format!("{} {}", d.name, palette.slate[9].paint(e.id()).bold())
                    })
                    .collect::<Vec<_>>();

                if !ids.is_empty() {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: ids.join(", "),
                    });
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "There's nothing here to peer at.".into(),
                    });
                }
            }
        }
    }
}
