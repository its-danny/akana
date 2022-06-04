use bevy::prelude::*;

use crate::{
    auth::components::authenticating::Authenticating,
    network::{events::NetworkEvent, server::NetworkServer},
    player::components::client::Client,
};

/// Spawn a new entity with a [`Player`] component when a new connection
/// comes in, an despawn it when the connection is lost.
pub fn handle_network_events(
    server: ResMut<NetworkServer>,
    mut commands: Commands,
    mut events: EventReader<NetworkEvent>,
    players: Query<(Entity, &Client)>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(id) => {
                commands.spawn_bundle((Client { id: *id, width: 80 }, Authenticating::default()));

                server.send("What's your name?", *id);

                info!("Player spawned for {id:?}");
            }
            NetworkEvent::Disconnected(id) => {
                if let Some((entity, _)) = players.iter().find(|(_, c)| c.id == *id) {
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
