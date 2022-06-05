use std::net::SocketAddr;

use tokio::net::TcpStream;

use super::{errors::NetworkError, server::ConnectionId};

#[derive(Debug)]
pub enum NetworkEvent {
    Connected(ConnectionId),
    Disconnected(ConnectionId),
    Error(NetworkError),
}

#[derive(Debug)]
pub struct IncomingConnection {
    pub socket: TcpStream,
    pub address: SocketAddr,
}

/// An array of [`TelnetCommand`]s to be sent to a client.
#[derive(Debug)]
pub struct NetworkCommand {
    pub command: [u8; 3],
}

#[derive(Debug)]
pub struct NetworkInput {
    pub id: ConnectionId,
    pub body: String,
    pub internal: bool,
}

#[derive(Debug)]
pub struct NetworkOutput {
    pub id: ConnectionId,
    pub body: String,
}
