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
        static ref CMD: Regex = Regex::new("^(say|')( )?(.+)?$").unwrap();
    }

    for message in input.iter() {
        if let Some(captures) = CMD.captures(&message.body) {
            if let Some((client, position, character)) =
                players.iter().find(|(c, _, _)| c.id == message.id)
            {
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
        test::bundles::utils::{connection_id, player_bundle},
    };

    #[test]
    fn say() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::say);

        let sender_id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(sender_id, "Igres", 0, 0));

        let recipient_id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(recipient_id, "Amri", 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: sender_id,
                body: "say Hey, Amri!".into(),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();

        let for_sender = output_reader
            .iter(output_events)
            .find(|o| o.id == sender_id)
            .unwrap();

        assert_eq!(for_sender.body, "You say \"Hey, Amri!\"");

        let for_recipient = output_reader
            .iter(output_events)
            .find(|o| o.id == recipient_id)
            .unwrap();

        assert_eq!(for_recipient.body, "Igres says \"Hey, Amri!\"");
    }

    #[test]
    fn nothing_to_say() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::say);

        let id = connection_id();
        app.world
            .spawn()
            .insert_bundle(player_bundle(id, "Igres", 0, 0));

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id,
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
