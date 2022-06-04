use bevy::prelude::*;

use crate::{
    network::events::NetworkMessage,
    player::{
        components::{client::Client, online::Online},
        events::prompt_event::PromptEvent,
    },
};

/// Any time we get new input from a player, we want to send
/// them their prompt.
pub fn emit_prompt_on_input(
    mut messages: EventReader<NetworkMessage>,
    mut prompts: EventWriter<PromptEvent>,
    players: Query<&Client, With<Online>>,
) {
    for event in messages.iter() {
        if let Some(client) = players.iter().find(|c| c.id == event.id) {
            prompts.send(PromptEvent(client.id));
        }
    }
}
