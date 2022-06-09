use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{character::Character, client::NetworkClient, online::Online},
    spatial::components::position::Position,
};

/// Broadcasts a message to anyone on the same tile as the sender.
pub fn say(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Position, &Character), With<Online>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(say|')(( +)(.+))?$").unwrap();
    }

    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body) {
            if let Some((client, position, character)) =
                players.iter().find(|(c, _, _)| c.id == message.id)
            {
                if let Some(phrase) = captures.get(4) {
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
                                    "{} says \"{}\"",
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

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};
    use yansi::Paint;

    use crate::{
        network::events::{NetworkInput, NetworkOutput},
        player::components::{character::Character, client::NetworkClient},
        test::bundles::utils::{player_bundle, PlayerBundle},
    };

    #[test]
    fn say() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::say);

        let sender = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let sender_client_id = app.world.get::<NetworkClient>(sender).unwrap().id;

        let recipient = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let recipient_client_id = app.world.get::<NetworkClient>(recipient).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: sender_client_id,
                body: "say Hey, Amri!".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();

        let for_sender = output_reader
            .iter(output_events)
            .find(|o| o.id == sender_client_id)
            .unwrap();

        assert_eq!(for_sender.body, "You say \"Hey, Amri!\"");

        let for_recipient = output_reader
            .iter(output_events)
            .find(|o| o.id == recipient_client_id)
            .unwrap();

        assert_eq!(
            for_recipient.body,
            format!(
                "{} says \"Hey, Amri!\"",
                app.world.get::<Character>(sender).unwrap().name
            )
        );
    }

    #[test]
    fn nothing_to_say() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::say);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: "say".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.body, "Say what?");
    }
}
