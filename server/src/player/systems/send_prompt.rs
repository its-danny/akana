use bevy::prelude::*;
use yansi::Paint;

use crate::{
    network::server::NetworkServer,
    player::{
        components::{character::Character, client::Client, online::Online},
        events::prompt_event::PromptEvent,
    },
    world::resources::world_time::{WorldTime, WorldTimeTag},
};

/// Send a prompt to anyone who needs it.
pub fn send_prompt(
    server: Res<NetworkServer>,
    world_time: Res<WorldTime>,
    mut prompts: EventReader<PromptEvent>,
    players: Query<(&Client, &Character), With<Online>>,
) {
    for event in prompts.iter() {
        if let Some((client, character)) = players.iter().find(|(c, _)| c.id == event.0) {
            let name = Paint::white(&character.name).bold();
            let time = world_time.time.format("%-l:%M%P").to_string();

            let world_status = "[{time}] >";
            let width = client.width as usize - character.name.len() - world_status.len();

            let world_status_colored = format!(
                "{}{}{} {}",
                "[",
                match world_time.part {
                    WorldTimeTag::Dawn | WorldTimeTag::Day => Paint::yellow(time),
                    WorldTimeTag::Dusk | WorldTimeTag::Night => Paint::blue(time),
                },
                "]",
                ">"
            );

            server.send(
                &format!("{:width$}{world_status_colored}", name,),
                client.id,
            );
        }
    }
}
