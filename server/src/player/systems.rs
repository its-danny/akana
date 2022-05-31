use bevy::prelude::*;
use colored::Colorize;

use crate::{
    auth::components::{Authenticating, Online},
    network::{
        events::{NetworkEvent, NetworkMessage},
        server::NetworkServer,
    },
    world::resources::{WorldTime, WorldTimePart},
};

use super::{
    components::{Character, Client},
    events::PromptEvent,
};

/// Spawn a new entity with a [`Player`] component when a new connection
/// comes in, an despawn it when the connection is lost.
pub(crate) fn handle_network_events(
    mut commands: Commands,
    mut events: EventReader<NetworkEvent>,
    players: Query<(Entity, &Client)>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(id) => {
                commands.spawn_bundle((Client { id: *id, width: 80 }, Authenticating::default()));

                info!("Player spawned for {id:?}");
            }
            NetworkEvent::Disconnected(id) => {
                if let Some((entity, _)) = players.iter().find(|p| p.1.id == *id) {
                    commands.entity(entity).despawn();

                    info!("Player despawned {id:?}");
                }
            }
            NetworkEvent::Error(error) => {
                error!("{error}");
            }
        };
    }
}

/// Any time we get new input from a player, we want to send
/// them their prompt.
pub(crate) fn send_prompt_on_input(
    mut messages: EventReader<NetworkMessage>,
    mut prompts: EventWriter<PromptEvent>,
    players: Query<(&Client, &Character), With<Online>>,
) {
    for event in messages.iter() {
        if let Some(player) = players.iter().find(|p| p.0.id == event.id) {
            prompts.send(PromptEvent(player.0.id));
        }
    }
}

/// Send a prompt to anyone who needs it.
pub(crate) fn send_prompt(
    server: Res<NetworkServer>,
    world_time: Res<WorldTime>,
    mut prompts: EventReader<PromptEvent>,
    players: Query<(&Client, &Character), With<Online>>,
) {
    for event in prompts.iter() {
        if let Some(player) = players.iter().find(|p| p.0.id == event.0) {
            let name = player.1.name.white().bold();
            let time = world_time.time.format("%-l:%M%P").to_string();

            let world_status = "[{time}] >";
            let width = player.0.width as usize - player.1.name.len() - world_status.len();

            let world_status_colored = format!(
                "{}{}{} {}",
                "[",
                match world_time.part {
                    WorldTimePart::Dawn | WorldTimePart::Day => time.yellow(),
                    WorldTimePart::Dusk | WorldTimePart::Night => time.blue(),
                },
                "]",
                ">"
            );

            server.send(&format!("{:width$}{world_status_colored}", name,), event.0);
        }
    }
}
