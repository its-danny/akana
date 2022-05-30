use crate::network::server::ConnectionId;

#[derive(Debug)]
pub(crate) struct PromptEvent(pub(crate) ConnectionId);
