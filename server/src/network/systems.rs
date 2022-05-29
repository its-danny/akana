use bevy::prelude::*;

use super::{events::NetworkEvent, messages::NetworkMessage, server::NetworkServer};

pub(crate) fn handle_incoming(server: Res<NetworkServer>) {
    for connection in server.incoming.receiver.try_iter() {
        debug!("Handling incoming connection: {connection:?}");

        server.setup_client(connection);
    }
}

pub(crate) fn handle_lost(server: Res<NetworkServer>) {
    for id in server.lost.receiver.try_iter() {
        debug!("Handling lost connection: {id:?}");

        server.remove_client(id);
    }
}

pub(crate) fn handle_events(server: Res<NetworkServer>, mut events: EventWriter<NetworkEvent>) {
    for event in server.events.receiver.try_iter() {
        debug!("Handling event: {event:?}");

        events.send(event);
    }
}

pub(crate) fn handle_inbox(server: Res<NetworkServer>, mut messages: EventWriter<NetworkMessage>) {
    for message in server.inbox.receiver.try_iter() {
        debug!("Handling inbox message: {message:?}");

        messages.send(message);
    }
}
