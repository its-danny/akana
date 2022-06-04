use crate::network::server::ConnectionId;

/// Read by the [`send_prompt`] system to send
/// prompts to players.
pub struct PromptEvent(pub ConnectionId);
