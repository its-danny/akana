use bevy::prelude::*;

#[derive(PartialEq)]
pub(crate) enum AuthState {
    Initial,
    AwaitingName,
    AwaitingPassword,
}

#[derive(Component)]
pub(crate) struct Authenticating {
    pub(crate) state: AuthState,
    pub(crate) name: String,
}

impl Default for Authenticating {
    fn default() -> Self {
        Self {
            state: AuthState::Initial,
            name: "".to_string(),
        }
    }
}

#[derive(Component)]
pub(crate) struct Online;
