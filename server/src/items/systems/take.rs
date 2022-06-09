use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    items::components::{backpack::Backpack, can_take::CanTake},
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::NetworkClient, online::Online},
    spatial::components::position::Position,
    visual::components::details::Details,
};

pub fn take(
    mut commands: Commands,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    mut players: Query<(&NetworkClient, &Position, &mut Backpack), With<Online>>,
    entities: Query<(Entity, &Position, &Details), With<CanTake>>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(take)(( +)(.+))?$").unwrap();
    }

    for message in input.iter() {
        if let Some((client, position, mut backpack)) =
            players.iter_mut().find(|(c, _, _)| c.id == message.id)
        {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                if backpack.0.len() >= 50 {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "You can't carry anything else!".into(),
                    });

                    break;
                }

                match captures.get(4) {
                    Some(name_or_id) => {
                        match entities.iter().find(|(e, p, d)| {
                            p.0 == position.0
                                && (d.name.to_lowercase()
                                    == name_or_id.as_str().to_lowercase().trim()
                                    || e.id().to_string() == name_or_id.as_str())
                        }) {
                            Some((entity, _, details)) => {
                                commands.entity(entity).remove::<Position>();
                                backpack.0.push(entity);

                                output.send(NetworkOutput {
                                    id: client.id,
                                    body: format!("You take the {}.", details.name),
                                });
                            }
                            None => {
                                output.send(NetworkOutput {
                                    id: client.id,
                                    body: "You don't see that here.".into(),
                                });
                            }
                        }
                    }
                    None => {
                        output.send(NetworkOutput {
                            id: client.id,
                            body: "Take what?".into(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};

    use crate::{
        items::components::backpack::Backpack,
        network::events::{NetworkInput, NetworkOutput},
        player::components::client::NetworkClient,
        spatial::components::position::Position,
        test::bundles::utils::{item_bundle, player_bundle, ItemBundle, PlayerBundle},
    };

    #[test]
    fn take() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::take);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        let item = app
            .world
            .spawn()
            .insert_bundle(item_bundle(ItemBundle {
                name: "Apple".into(),
                ..Default::default()
            }))
            .id();

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: format!("take apple"),
                internal: false,
            });

        app.update();

        assert!(app.world.get::<Position>(item).is_none());
        assert_eq!(
            app.world.get::<Backpack>(player).unwrap().0.get(0).unwrap(),
            &item
        );

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You take the Apple.");
    }

    #[test]
    fn entity_not_found() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::take);

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: format!("take apple"),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You don't see that here.");
    }
}
