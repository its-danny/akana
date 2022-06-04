use api::auth::handlers::SignInResponse;
use bevy::prelude::*;
use reqwest::StatusCode;

use crate::{
    auth::{
        components::authenticating::{AuthState, Authenticating},
        utils::api::{sign_in, user_exists},
    },
    network::{
        events::NetworkMessage,
        server::{NetworkServer, TelnetCommand::*},
    },
    player::{
        components::{character::Character, client::Client, online::Online},
        events::prompt_event::PromptEvent,
    },
    spatial::components::position::Position,
    world::resources::new_player_spawn::NewPlayerSpawn,
};

/// Intercept all [`NetworkMessage`] for any user that's currently
/// authenticating and handle authentication.
pub fn perform_authentication(
    mut commands: Commands,
    server: Res<NetworkServer>,
    new_player_spawn: Res<NewPlayerSpawn>,
    mut messages: EventReader<NetworkMessage>,
    mut prompts: EventWriter<PromptEvent>,
    mut players: Query<(Entity, &Client, &mut Authenticating)>,
) {
    for message in messages.iter() {
        if let Some((entity, client, mut authenticating)) =
            players.iter_mut().find(|(_, c, _)| c.id == message.id)
        {
            match authenticating.state {
                AuthState::AwaitingName => {
                    // Validate the name. Currently, that just means it's
                    // more than 3 letters long.
                    if message.body.len() < 3 {
                        server.send_message("That's not a valid name. Try again!", client.id);

                        break;
                    }

                    match user_exists(message.body.clone()).status() {
                        StatusCode::FOUND => {
                            // If the user already exists we want to skip straight to asking
                            // for their password. We send this telnet command along with it
                            // so that their client won't echo back their passwor.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], client.id);
                            server.send_message("What's your password?", client.id);
                        }
                        StatusCode::NOT_FOUND => {
                            // If the user is not found, we do the same, but letting them
                            // know this will be a new account.
                            server.send_command([Iac as u8, Will as u8, Echo as u8], client.id);
                            server.send_message("Looks like this is a new character. What password would you like to use?", client.id);
                        }
                        _ => {
                            server.send_message("Uh oh, something broke!", client.id);
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
                        server.send_message("That's not a valid password. Try again!", client.id);

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
                            server.send_message("Authenticated.", client.id);

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
                                Position(new_player_spawn.0),
                            ));
                        }
                        StatusCode::FORBIDDEN => {
                            // If their password was not correct, let them try again.
                            server.send_message("Incorrect password. Try again!", client.id);
                        }
                        _ => {
                            server.send_message("Uh oh, something broke!", client.id);
                        }
                    }
                }
            }
        }
    }
}
