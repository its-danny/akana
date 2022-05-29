use super::server::ConnectionId;

#[derive(Debug)]
pub(crate) struct NetworkMessage {
    pub(crate) id: ConnectionId,
    pub(crate) command: Option<[u8; 3]>,
    pub(crate) body: String,
}
