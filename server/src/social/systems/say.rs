use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::{character::Character, client::Client, online::Online},
    spatial::components::position::Position,
};

/// Broadcasts a message to anyone on the same tile as the sender.
pub fn say(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<(&Client, &Position, &Character), With<Online>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(say( )?|')(.+)?$").unwrap();
    }

    for message in messages.iter() {
        if let Some((client, position, character)) =
            players.iter().find(|(c, _, _)| c.id == message.id)
        {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if let Some(phrase) = captures.get(3) {
                    server.send_message(
                        &format!("You say \"{}\"", Paint::white(phrase.as_str()).bold()),
                        client.id,
                    );

                    players
                        .iter()
                        .filter(|(c, p, _)| p.0 == position.0 && c.id != client.id)
                        .for_each(|(c, _, _)| {
                            server.send_message(
                                &format!(
                                    "{} said \"{}\"",
                                    Paint::cyan(&character.name),
                                    Paint::white(phrase.as_str().trim()).bold()
                                ),
                                c.id,
                            )
                        });
                } else {
                    server.send_message("Say what?", client.id);
                }
            }
        }
    }
}
