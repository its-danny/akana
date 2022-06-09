use bevy::prelude::*;

use crate::{
    auth::components::authenticating::Authenticating,
    network::events::{NetworkEvent, NetworkOutput},
    player::components::client::NetworkClient,
};

/// Spawn a new entity with a [`Player`] component when a new connection
/// comes in, an despawn it when the connection is lost.
pub fn handle_network_events(
    mut commands: Commands,
    mut events: EventReader<NetworkEvent>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(Entity, &NetworkClient)>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(id) => {
                commands.spawn_bundle((
                    NetworkClient { id: *id, width: 80 },
                    Authenticating::default(),
                ));

                output.send(NetworkOutput {
                    id: *id,
                    body: "What's your name?".to_string(),
                });

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
