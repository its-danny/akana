pub(crate) mod components;
pub(crate) mod events;
pub(crate) mod systems;

use bevy::prelude::*;

use self::{
    events::PromptEvent,
    systems::{handle_network_events, send_prompt, send_prompt_on_input},
};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PromptEvent>();

        app.add_system_set(
            SystemSet::new()
                .label("player")
                .with_system(handle_network_events)
                .with_system(send_prompt_on_input)
                .with_system(send_prompt),
        );
    }
}
