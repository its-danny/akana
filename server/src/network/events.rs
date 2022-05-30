use std::net::SocketAddr;

use tokio::net::TcpStream;

use super::{errors::NetworkError, server::ConnectionId};

#[derive(Debug)]
pub(crate) enum NetworkEvent {
    Connected(ConnectionId),
    Disconnected(ConnectionId),
    Error(NetworkError),
}

#[derive(Debug)]
pub(crate) struct NetworkCommand {
    pub(crate) command: [u8; 3],
}

#[derive(Debug)]
pub(crate) struct NetworkMessage {
    pub(crate) id: ConnectionId,
    pub(crate) body: String,
}

#[derive(Debug)]
pub(crate) struct IncomingConnection {
    pub(crate) socket: TcpStream,
    pub(crate) address: SocketAddr,
}
