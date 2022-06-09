use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use yansi::Paint;

use crate::{
    network::events::{NetworkInput, NetworkOutput},
    player::components::{character::Character, client::NetworkClient, online::Online},
    spatial::components::position::Position,
};

lazy_static! {
    static ref CMD: Regex = Regex::new("^(emote|;)(( +)(.+))?$").unwrap();
}

/// Broadcasts a message to anyone on the same tile as the sender.
pub fn emote(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Position, &Character), With<Online>>,
) {
    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body) {
            if let Some((client, position, character)) =
                players.iter().find(|(c, _, _)| c.id == message.id)
            {
                if let Some(phrase) = captures.get(4) {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: format!(
                            "{} {}",
                            Paint::cyan(&character.name),
                            Paint::white(phrase.as_str().trim())
                        ),
                    });

                    players
                        .iter()
                        .filter(|(c, p, _)| p.0 == position.0 && c.id != client.id)
                        .for_each(|(c, _, _)| {
                            output.send(NetworkOutput {
                                id: c.id,
                                body: format!(
                                    "{} {}",
                                    Paint::cyan(&character.name),
                                    Paint::white(phrase.as_str().trim())
                                ),
                            });
                        });
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "Emote what?".to_string(),
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
    fn emote() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::emote);

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
                body: "emote sighs".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();

        let for_sender = output_reader
            .iter(output_events)
            .find(|o| o.id == sender_client_id)
            .unwrap();

        assert_eq!(
            for_sender.body,
            format!("{} sighs", app.world.get::<Character>(sender).unwrap().name)
        );

        let for_recipient = output_reader
            .iter(output_events)
            .find(|o| o.id == recipient_client_id)
            .unwrap();

        assert_eq!(
            for_recipient.body,
            format!("{} sighs", app.world.get::<Character>(sender).unwrap().name)
        );
    }

    #[test]
    fn nothing_to_say() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::emote);

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
                body: "emote".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.body, "Emote what?");
    }
}
