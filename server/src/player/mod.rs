pub(crate) mod components;
pub(crate) mod events;
pub(crate) mod systems;

use bevy::prelude::*;

use self::{
    events::SendPrompt,
    systems::{handle_network_events, handle_network_message, handle_send_prompt},
};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendPrompt>();
        app.add_system(handle_network_events);
        app.add_system(handle_network_message);
        app.add_system(handle_send_prompt);
    }
}
