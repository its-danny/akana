use crate::network::server::ConnectionId;

#[derive(Debug)]
pub(crate) struct SendPrompt {
    pub(crate) id: ConnectionId,
    pub(crate) name: String,
}
