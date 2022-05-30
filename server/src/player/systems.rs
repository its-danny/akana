use bevy::prelude::*;

use crate::{
    auth::components::{Authenticating, Online},
    network::{
        events::{NetworkEvent, NetworkMessage},
        server::NetworkServer,
    },
};

use super::{
    components::{Character, Client},
    events::SendPrompt,
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
                commands.spawn_bundle((Client(*id), Authenticating::default()));

                info!("Player spawned for {id:?}");
            }
            NetworkEvent::Disconnected(id) => {
                if let Some((entity, _)) = players.iter().find(|p| p.1 .0 == *id) {
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
pub(crate) fn handle_network_message(
    mut message_events: EventReader<NetworkMessage>,
    mut prompt_events: EventWriter<SendPrompt>,
    players: Query<(&Client, &Character), With<Online>>,
) {
    for event in message_events.iter() {
        if let Some(player) = players.iter().find(|p| p.0 .0 == event.id) {
            prompt_events.send(SendPrompt(player.0 .0));
        }
    }
}

/// Send a prompt to anyone who needs it.
pub(crate) fn handle_send_prompt(
    server: Res<NetworkServer>,
    mut events: EventReader<SendPrompt>,
    players: Query<(&Client, &Character), With<Online>>,
) {
    for event in events.iter() {
        if let Some(player) = players.iter().find(|p| p.0 .0 == event.0) {
            server.send(&format!("{} >", player.1.name), event.0);
        }
    }
}
