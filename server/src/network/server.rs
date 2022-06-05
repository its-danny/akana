use std::{net::SocketAddr, sync::Arc};

use bevy::{prelude::*, utils::Uuid};
use dashmap::DashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, ToSocketAddrs},
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

use super::{
    errors::NetworkError,
    events::{IncomingConnection, NetworkCommand, NetworkEvent, NetworkInput, NetworkOutput},
    SyncChannel,
};

pub enum TelnetCommand {
    Iac = 255,
    Will = 251,
    Wont = 252,
    Echo = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionId {
    pub uuid: Uuid,
    address: SocketAddr,
}

struct ClientConnection {
    #[allow(dead_code)]
    read_task: JoinHandle<()>,
    #[allow(dead_code)]
    write_task: JoinHandle<()>,
    /// Messages to be sent out
    outbox: SyncChannel<(Option<NetworkCommand>, Option<NetworkOutput>)>,
}

pub struct NetworkServer {
    runtime: Runtime,
    /// Incoming connections
    pub incoming: SyncChannel<IncomingConnection>,
    /// Active clients
    clients: Arc<DashMap<ConnectionId, ClientConnection>>,
    /// Recently disconnected clients
    pub lost: SyncChannel<ConnectionId>,
    /// Network events
    pub events: SyncChannel<NetworkEvent>,
    /// Messages received from clients
    pub inbox: SyncChannel<NetworkInput>,
}

impl NetworkServer {
    pub fn new() -> Self {
        Self {
            runtime: Builder::new_multi_thread()
                .enable_io()
                .build()
                .expect("Could not build runtime"),
            incoming: SyncChannel::new(),
            clients: Arc::new(DashMap::new()),
            lost: SyncChannel::new(),
            events: SyncChannel::new(),
            inbox: SyncChannel::new(),
        }
    }

    pub fn listen(&self, address: impl ToSocketAddrs + Send + 'static) {
        let incoming = self.incoming.sender.clone();
        let events = self.events.sender.clone();

        self.runtime.spawn(async move {
            let listener = match TcpListener::bind(address).await {
                Ok(listener) => listener,
                Err(error) => {
                    if let Err(error) =
                        events.send(NetworkEvent::Error(NetworkError::Listen(error)))
                    {
                        error!("Could not send error: {error}");
                    };

                    return;
                }
            };

            loop {
                match listener.accept().await {
                    Ok((socket, address)) => {
                        info!("Incoming connection from {address}");

                        if let Err(error) = incoming.send(IncomingConnection { socket, address }) {
                            error!("Could not send incoming connection: {error}");
                        }
                    }
                    Err(error) => {
                        if let Err(error) =
                            events.send(NetworkEvent::Error(NetworkError::Accept(error)))
                        {
                            error!("Could not send error: {error}");
                        };
                    }
                };
            }
        });
    }

    pub fn setup_client(&self, connection: IncomingConnection) {
        let outbox: SyncChannel<(Option<NetworkCommand>, Option<NetworkOutput>)> =
            SyncChannel::new();

        let lost_sender = self.lost.sender.clone();
        let read_events_sender = self.events.sender.clone();
        let write_events_sender = self.events.sender.clone();
        let inbox_sender = self.inbox.sender.clone();
        let outbox_receiver = outbox.receiver.clone();

        let (mut read_socket, mut write_socket) = connection.socket.into_split();

        let id = ConnectionId {
            uuid: Uuid::new_v4(),
            address: connection.address,
        };

        if let Err(error) = read_events_sender.send(NetworkEvent::Connected(id)) {
            error!("Could not send event: {error}");
        }

        self.clients.insert(
            id,
            ClientConnection {
                outbox,
                read_task: self.runtime.spawn(async move {
                    let max_packet_size = 1024;
                    let mut buffer = vec![0; max_packet_size];

                    debug!("Starting listen task for {id:?}");

                    loop {
                        let length = match read_socket.read(&mut buffer).await {
                            Ok(n) => n,
                            Err(error) => {
                                if let Err(error) = read_events_sender
                                    .send(NetworkEvent::Error(NetworkError::SocketRead(error, id)))
                                {
                                    error!("Could not send error: {error}");
                                };

                                return;
                            }
                        };

                        if length == 0 {
                            if let Err(error) = lost_sender.send(id) {
                                error!("Could not send lost connection: {error}");
                            }

                            break;
                        }

                        let message = std::str::from_utf8(&buffer[..length])
                            .unwrap_or("")
                            .trim()
                            .to_string();

                        if !message.is_empty() {
                            if let Err(error) = inbox_sender.send(NetworkInput {
                                id,
                                body: message,
                                internal: false,
                            }) {
                                error!("Could not send to inbox: {error}");
                            }
                        }
                    }
                }),
                write_task: self.runtime.spawn(async move {
                    while let Ok(output) = outbox_receiver.recv() {
                        if let Some(command) = output.0 {
                            match write_socket.write_all(&command.command).await {
                                Ok(_) => (),
                                Err(error) => {
                                    if let Err(error) = write_events_sender.send(
                                        NetworkEvent::Error(NetworkError::SocketWrite(error, id)),
                                    ) {
                                        error!("Could not send error: {error}");
                                    };

                                    return;
                                }
                            }
                        }

                        if let Some(message) = output.1 {
                            match write_socket.write_all(message.body.as_bytes()).await {
                                Ok(_) => (),
                                Err(error) => {
                                    if let Err(error) = write_events_sender.send(
                                        NetworkEvent::Error(NetworkError::SocketWrite(error, id)),
                                    ) {
                                        error!("Could not send error: {error}");
                                    };

                                    return;
                                }
                            }
                        }
                    }
                }),
            },
        );
    }

    pub fn remove_client(&self, id: ConnectionId) {
        self.clients.remove(&id);

        if let Err(error) = self.events.sender.send(NetworkEvent::Disconnected(id)) {
            error!("Could not send event: {error}");
        }

        info!("Client disconnected: {id:?}");
    }

    pub fn send_message(&self, message: &str, id: ConnectionId) {
        info!("Sending message to {id:?}: {message:?}");

        if let Some(client) = self.clients.get(&id) {
            if let Err(error) = client.value().outbox.sender.send((
                None,
                Some(NetworkOutput {
                    id,
                    body: format!("{message}\r\n"),
                }),
            )) {
                error!("Could not send to outbox: {error}");
            }
        }
    }

    pub fn send_command(&self, command: [u8; 3], id: ConnectionId) {
        info!("Sending command to {id:?}: {command:?}");

        if let Some(client) = self.clients.get(&id) {
            if let Err(error) = client
                .value()
                .outbox
                .sender
                .send((Some(NetworkCommand { command }), None))
            {
                error!("Could not send to outbox: {error}");
            }
        }
    }
}
