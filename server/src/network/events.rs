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
///
/// If the message is internal, it has been sent by
/// the server to the server, pretending to be a client.
/// This is used to trigger commands without the need
/// for a player to send them. An example being `look` after you
/// move somewhere.
#[derive(Debug)]
pub struct NetworkMessage {
    pub id: ConnectionId,
    pub body: String,
    pub internal: bool,
}

#[derive(Debug)]
pub struct IncomingConnection {
    pub socket: TcpStream,
    pub address: SocketAddr,
}
