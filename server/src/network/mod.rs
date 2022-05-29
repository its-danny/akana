mod errors;
pub(crate) mod events;
pub(crate) mod messages;
pub(crate) mod server;
mod systems;

use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use events::NetworkEvent;
use messages::NetworkMessage;
use server::NetworkServer;
use systems::{handle_events, handle_inbox, handle_incoming, handle_lost};

pub(crate) struct SyncChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> SyncChannel<T> {
    fn new() -> Self {
        let (sender, receiver) = unbounded();

        Self { sender, receiver }
    }
}

pub(crate) struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkServer::new());
        app.add_event::<NetworkEvent>();
        app.add_event::<NetworkMessage>();
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(handle_incoming)
                .with_system(handle_lost)
                .with_system(handle_events)
                .with_system(handle_inbox),
        );
    }
}
