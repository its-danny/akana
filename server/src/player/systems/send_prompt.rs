use bevy::prelude::*;
use yansi::Paint;

use crate::{
    network::events::NetworkOutput,
    player::{
        components::{character::Character, client::NetworkClient, online::Online},
        events::prompt_event::PromptEvent,
    },
    world::resources::world_time::{WorldTime, WorldTimeTag},
};

/// Send a prompt to anyone who needs it.
pub fn send_prompt(
    world_time: Res<WorldTime>,
    mut prompts: EventReader<PromptEvent>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Character), With<Online>>,
) {
    for event in prompts.iter() {
        if let Some((client, character)) = players.iter().find(|(c, _)| c.id == event.0) {
            let time = world_time.time.format("%-l:%M%P").to_string();

            let prompt = format!(
                "{} [{}] >",
                Paint::white(&character.name).bold(),
                match world_time.part {
                    WorldTimeTag::Dawn | WorldTimeTag::Day => Paint::yellow(time),
                    WorldTimeTag::Dusk | WorldTimeTag::Night => Paint::blue(time),
                },
            );

            output.send(NetworkOutput {
                id: client.id,
                body: prompt,
            });
        }
    }
}
