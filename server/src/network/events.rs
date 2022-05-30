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
pub(crate) struct NetworkMessage {
    pub(crate) id: ConnectionId,
    pub(crate) command: Option<[u8; 3]>,
    pub(crate) body: String,
}

#[derive(Debug)]
pub(crate) struct IncomingConnection {
    pub(crate) socket: TcpStream,
    pub(crate) address: SocketAddr,
}
