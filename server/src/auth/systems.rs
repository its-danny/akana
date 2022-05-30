use api::auth::handlers::SignInResponse;
use bevy::prelude::*;
use reqwest::StatusCode;

use crate::{
    network::{
        events::NetworkMessage,
        server::NetworkServer,
        server::TelnetCommand::{Echo, Iac, Will, Wont},
    },
    player::{
        components::{Account, Character, Client},
        events::SendPrompt,
    },
    spatial::components::Position,
};

use super::{
    api::{sign_in, user_exists},
    components::{AuthState, Authenticating, Online},
};

/// When a user first connects, we give them the [`Authenticating`]
/// component. This system gets anyone with that component
/// and begins the authentication process.
pub(crate) fn start_authenticating_new_clients(
    server: Res<NetworkServer>,
    mut players: Query<(&Client, &mut Authenticating)>,
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
pub(crate) fn handle_network_message(
    mut commands: Commands,
    server: Res<NetworkServer>,
    mut messages: EventReader<NetworkMessage>,
    mut prompt_events: EventWriter<SendPrompt>,
    mut players: Query<(Entity, &Client, &mut Authenticating)>,
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

                    let response = sign_in(authenticating.name.clone(), message.body.clone());

                    match response.status() {
                        StatusCode::OK => {
                            let json = response
                                .json::<SignInResponse>()
                                .expect("Could not parse response");

                            // If the user's password is correct, let them know and
                            // send another telnet command letting their client know it's
                            // ok to echo input again.
                            server.send_command([Iac as u8, Wont as u8, Echo as u8], player.0);
                            server.send("Authenticated.", player.0);

                            // Send the prompt.
                            prompt_events.send(SendPrompt(player.0));

                            // Remove `Authenticating` now that we're done.
                            commands.entity(entity).remove::<Authenticating>();

                            // Set the player up.
                            commands.entity(entity).insert_bundle((
                                Online,
                                Account(json.id),
                                Character { name: json.name },
                                Position((0, 0, 0)),
                            ));
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
