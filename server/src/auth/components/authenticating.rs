use bevy::prelude::*;

#[derive(Component)]
pub struct Authenticating {
    pub state: AuthState,
    pub name: String,
}

impl Default for Authenticating {
    fn default() -> Self {
        Self {
            state: AuthState::AwaitingName,
            name: "".to_string(),
        }
    }
}

#[derive(PartialEq)]
pub enum AuthState {
    AwaitingName,
    AwaitingPassword,
}
