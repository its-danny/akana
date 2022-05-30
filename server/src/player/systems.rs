use bevy::prelude::*;

use crate::{auth::components::Authenticating, network::events::NetworkEvent};

use super::components::Client;

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
