use api::auth::handlers::SignInResponse;
use bevy::prelude::*;
use reqwest::StatusCode;

use crate::{
    auth::{
        components::authenticating::{AuthState, Authenticating},
        utils::api::{sign_in, user_exists},
    },
    items::components::backpack::Backpack,
    network::{
        events::{NetworkInput, NetworkOutput},
        server::{NetworkServer, TelnetCommand::*},
    },
    player::{
        components::{character::Character, client::NetworkClient, online::Online},
        events::prompt_event::PromptEvent,
    },
    spatial::components::position::Position,
    world::resources::new_player_spawn::NewPlayerSpawn,
};

/// Intercept all [`NetworkInput`] for any user that's currently
/// authenticating and handle authentication.
pub fn perform_authentication(
    mut commands: Commands,
    server: Res<NetworkServer>,
    new_player_spawn: Res<NewPlayerSpawn>,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    mut prompts: EventWriter<PromptEvent>,
    mut players: Query<(Entity, &NetworkClient, &mut Authenticating)>,
) {
    for message in input.iter() {
        if let Some((entity, client, mut authenticating)) =
            players.iter_mut().find(|(_, c, _)| c.id == message.id)
        {
            match authenticating.state {
                AuthState::AwaitingName => {
                    // Validate the name. Currently, that just means it's
                    // more than 3 letters long.
                    if message.body.len() < 3 {
                        output.send(NetworkOutput {
                            id: client.id,
                            body: "That's not a valid name. Try again!".to_string(),
                        });

                        break;
                    }

                    match user_exists(message.body.clone()).status() {
                        StatusCode::FOUND => {
                            // If the user already exists we want to skip straight to asking
                            // for their password. We send this telnet command along with it
                            // so that their client won't echo back their passwor.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], client.id);

                            output.send(NetworkOutput {
                                id: client.id,
                                body: "What's your password?".to_string(),
                            });
                        }
                        StatusCode::NOT_FOUND => {
                            // If the user is not found, we do the same, but letting them
                            // know this will be a new account.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], client.id);

                            output.send(NetworkOutput {
                                id: client.id,
                                body: "Looks like this is a new character. What password would you like to use?".to_string(),
                            });
                        }
                        _ => {
                            output.send(NetworkOutput {
                                id: client.id,
                                body: "Uh oh, something broke!".to_string(),
                            });
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
                        output.send(NetworkOutput {
                            id: client.id,
                            body: "That's not a valid password. Try again!".to_string(),
                        });

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
                            server.send_command([Iac as u8, Wont as u8, Echo as u8], client.id);

                            output.send(NetworkOutput {
                                id: client.id,
                                body: "Authenticated.".to_string(),
                            });

                            // Send the prompt.
                            prompts.send(PromptEvent(client.id));

                            // Remove `Authenticating` now that we're done.
                            commands.entity(entity).remove::<Authenticating>();

                            // Set the player up.
                            commands.entity(entity).insert_bundle((
                                Online,
                                Character {
                                    id: json.id,
                                    name: json.name,
                                },
                                Backpack(Vec::new()),
                                Position(new_player_spawn.0),
                            ));
                        }
                        StatusCode::FORBIDDEN => {
                            // If their password was not correct, let them try again.
                            output.send(NetworkOutput {
                                id: client.id,
                                body: "Incorrect password. Try again!".to_string(),
                            });
                        }
                        _ => {
                            output.send(NetworkOutput {
                                id: client.id,
                                body: "Uh oh, something broke!".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}
