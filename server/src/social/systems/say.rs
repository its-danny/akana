use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{character::Character, client::Client, online::Online},
    spatial::components::position::Position,
};

/// Broadcasts a message to anyone on the same tile as the sender.
pub fn say(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&Client, &Position, &Character), With<Online>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(say( )?|')(.+)?$").unwrap();
    }

    for message in input.iter() {
        if let Some((client, position, character)) =
            players.iter().find(|(c, _, _)| c.id == message.id)
        {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if let Some(phrase) = captures.get(3) {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: format!("You say \"{}\"", Paint::white(phrase.as_str()).bold()),
                    });

                    players
                        .iter()
                        .filter(|(c, p, _)| p.0 == position.0 && c.id != client.id)
                        .for_each(|(c, _, _)| {
                            output.send(NetworkOutput {
                                id: c.id,
                                body: format!(
                                    "{} said \"{}\"",
                                    Paint::cyan(&character.name),
                                    Paint::white(phrase.as_str().trim()).bold()
                                ),
                            });
                        });
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "Say what?".to_string(),
                    });
                }
            }
        }
    }
}
