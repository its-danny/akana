use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    auth::components::Online,
    network::{events::NetworkMessage, server::NetworkServer},
    player::components::{Character, Client},
    spatial::components::Position,
};

/// Broadcasts a message to anyone on the same tile as the sender.
pub(crate) fn say(
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    players: Query<(&Client, &Position, &Character), With<Online>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(say( )?|')(.+)?$").unwrap();
    }

    for message in messages.iter() {
        if let Some(player) = players.iter().find(|p| p.0.id == message.id) {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if let Some(phrase) = captures.get(3) {
                    server.send(
                        &format!("You say \"{}\"", Paint::white(phrase.as_str()).bold()),
                        player.0.id,
                    );

                    players
                        .iter()
                        .filter(|p| p.1 .0 == player.1 .0 && p.0.id != player.0.id)
                        .for_each(|p| {
                            server.send(
                                &format!(
                                    "{} said \"{}\"",
                                    Paint::cyan(&player.2.name),
                                    Paint::white(phrase.as_str().trim()).bold()
                                ),
                                p.0.id,
                            )
                        });
                } else {
                    server.send("Say what?", player.0.id);
                }
            }
        }
    }
}
