use std::net::SocketAddr;

use tokio::net::TcpStream;

use super::{errors::NetworkError, server::ConnectionId};

#[derive(Debug)]
pub enum NetworkEvent {
    Connected(ConnectionId),
    Disconnected(ConnectionId),
    Error(NetworkError),
}

/// An array of [`TelnetCommand`]s to be sent to a client.
#[derive(Debug)]
pub struct NetworkCommand {
    pub command: [u8; 3],
}

/// Both inbound and outbound messages between the
/// game server and a client.
#[derive(Debug)]
pub struct NetworkMessage {
    pub id: ConnectionId,
    pub body: String,
}

#[derive(Debug)]
pub struct IncomingConnection {
    pub socket: TcpStream,
    pub address: SocketAddr,
}
