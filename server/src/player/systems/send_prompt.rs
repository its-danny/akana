use bevy::prelude::*;

use crate::{
    network::events::NetworkOutput,
    player::{
        components::{character::Character, client::NetworkClient, online::Online},
        events::prompt_event::PromptEvent,
    },
    visual::palette::Palette,
    world::resources::world_time::{WorldTime, WorldTimeTag},
};

/// Send a prompt to anyone who needs it.
pub fn send_prompt(
    world_time: Res<WorldTime>,
    palette: Res<Palette>,
    mut prompts: EventReader<PromptEvent>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Character), With<Online>>,
) {
    for event in prompts.iter() {
        if let Some((client, character)) = players.iter().find(|(c, _)| c.id == event.0) {
            let time = world_time.time.format("%-l:%M%P").to_string();

            let prompt = format!(
                "{} [{}] >",
                palette.neutral[0].paint(&character.name).bold(),
                match world_time.part {
                    WorldTimeTag::Dawn | WorldTimeTag::Day => palette.yellow[2].paint(time),
                    WorldTimeTag::Dusk | WorldTimeTag::Night => palette.purple[8].paint(time),
                },
            );

            output.send(NetworkOutput {
                id: client.id,
                body: prompt,
            });
        }
    }
}
