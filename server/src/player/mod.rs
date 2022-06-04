pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use self::{
    events::prompt_event::PromptEvent,
    systems::{emit_prompt_on_input::*, handle_network_events::*, send_prompt::*},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PromptEvent>();

        app.add_system_set(
            SystemSet::new()
                .label("player")
                .with_system(handle_network_events)
                .with_system(emit_prompt_on_input)
                .with_system(send_prompt),
        );
    }
}
