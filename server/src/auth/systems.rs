use bevy::prelude::*;
use reqwest::StatusCode;

use crate::{
    network::{
        messages::NetworkMessage,
        server::NetworkServer,
        server::TelnetCommand::{Echo, Iac, Will, Wont},
    },
    player::components::Player,
};

use super::{
    components::{AuthState, Authenticating, Online},
    utils::{sign_in, user_exists},
};

/// When a user first connects, we give them the [`Authenticating`]
/// component. This system gets anyone with that component
/// and begins the authentication process.
pub(crate) fn start_authenticating(
    server: Res<NetworkServer>,
    mut players: Query<(&Player, &mut Authenticating)>,
) {
    for (player, mut authenticating) in players.iter_mut() {
        if authenticating.state == AuthState::Initial {
            authenticating.state = AuthState::AwaitingName;

            server.send("What's your name?", player.0);
        }
    }
}

/// Intercept all [`NetworkMessage`] for any user that's currently
/// authenticating and handle authentication.
pub(crate) fn handle_network_messages(
    mut commands: Commands,
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    mut players: Query<(Entity, &Player, &mut Authenticating)>,
) {
    for message in messages.iter() {
        if let Some((entity, player, mut authenticating)) =
            players.iter_mut().find(|p| p.1 .0 == message.id)
        {
            match authenticating.state {
                AuthState::AwaitingName => {
                    // Validate the name. Currently, that just means it's
                    // more than 3 letters long.
                    if message.body.len() < 3 {
                        server.send("That's not a valid name. Try again!", player.0);

                        break;
                    }

                    match user_exists(message.body.clone()).status() {
                        StatusCode::FOUND => {
                            // If the user already exists we want to skip straight to asking
                            // for their password. We send this telnet command along with it
                            // so that their client won't echo back their passwor.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], player.0);
                            server.send("What's your your password?", player.0);
                        }
                        StatusCode::NOT_FOUND => {
                            // If the user is not found, we do the same, but letting them
                            // know this will be a new account.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], player.0);
                            server.send("Looks like this is a new character. What password would you like to use?", player.0);
                        }
                        _ => {
                            server.send("Uh oh, something broke!", player.0);
                        }
                    }

                    // Update the `Authenticating` component with the data necessary
                    // for the next step.
                    authenticating.name = message.body.clone();
                    authenticating.state = AuthState::AwaitingPassword;
                }
                AuthState::AwaitingPassword => {
                    // Validate the name. Currently, that just means it's
                    // more than 3 letters long.
                    if message.body.len() < 3 {
                        server.send("That's not a valid password. Try again!", player.0);

                        break;
                    }

                    match sign_in(authenticating.name.clone(), message.body.clone()).status() {
                        StatusCode::OK => {
                            // If the user's password is correct, let them know and
                            // send another telnet command letting their client know it's
                            // ok to echo input again.
                            server.send_command([Iac as u8, Wont as u8, Echo as u8], player.0);
                            server.send("Authenticated.", player.0);

                            // Finish up the authentication process.
                            commands.entity(entity).remove::<Authenticating>();
                            commands.entity(entity).insert(Online);
                        }
                        StatusCode::FORBIDDEN => {
                            // If their password was not correct, let them try again.
                            server.send("Incorrect password. Try again!", player.0);
                        }
                        _ => {
                            server.send("Uh oh, something broke!", player.0);
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
