use bevy::prelude::*;

use crate::{
    network::events::NetworkInput,
    player::{
        components::{client::NetworkClient, online::Online},
        events::prompt_event::PromptEvent,
    },
};

/// Any time we get new input from a player, we want to send
/// them their prompt.
pub fn emit_prompt_on_input(
    mut input: EventReader<NetworkInput>,
    mut prompts: EventWriter<PromptEvent>,
    players: Query<&NetworkClient, With<Online>>,
) {
    for message in input.iter() {
        if !message.internal {
            if let Some(client) = players.iter().find(|c| c.id == message.id) {
                prompts.send(PromptEvent(client.id));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::{ecs::event::Events, prelude::*};

    use crate::{
        network::events::NetworkInput,
        player::{components::client::NetworkClient, events::prompt_event::PromptEvent},
        test::bundles::utils::{player_bundle, PlayerBundle},
    };

    #[test]
    fn emits() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<PromptEvent>();
        app.add_system(super::emit_prompt_on_input);

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
                body: "look".into(),
                internal: false,
            });

        app.update();

        let prompt_events = app.world.resource::<Events<PromptEvent>>();
        let mut prompt_reader = prompt_events.get_reader();
        let prompt = prompt_reader.iter(prompt_events).next().unwrap();

        assert_eq!(prompt.0, player_client_id);
    }
}
