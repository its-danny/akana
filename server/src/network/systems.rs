use std::env;

use bevy::prelude::*;

use super::{
    events::{NetworkEvent, NetworkInput, NetworkOutput},
    server::NetworkServer,
};

pub fn setup_network(server: Res<NetworkServer>) {
    let server_url = env::var("SERVER_URL").expect("Could not read SERVER_URL from env");

    server.listen(server_url);
}

pub fn handle_incoming(server: Res<NetworkServer>) {
    for connection in server.incoming.receiver.try_iter() {
        debug!("Handling incoming connection: {connection:?}");

        server.setup_client(connection);
    }
}

pub fn handle_lost(server: Res<NetworkServer>) {
    for id in server.lost.receiver.try_iter() {
        debug!("Handling lost connection: {id:?}");

        server.remove_client(id);
    }
}

pub fn handle_events(server: Res<NetworkServer>, mut events: EventWriter<NetworkEvent>) {
    for event in server.events.receiver.try_iter() {
        debug!("Handling event: {event:?}");

        events.send(event);
    }
}

pub fn handle_inbox(server: Res<NetworkServer>, mut input: EventWriter<NetworkInput>) {
    for message in server.inbox.receiver.try_iter() {
        debug!("Handling inbox message: {message:?}");

        input.send(message);
    }
}

pub fn handle_outbox(server: Res<NetworkServer>, mut output: EventReader<NetworkOutput>) {
    for message in output.iter() {
        server.send_message(&message.body, message.id);
    }
}
