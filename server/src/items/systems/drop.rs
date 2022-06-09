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

/// Drop an item from an entitys backpack.
///
/// We do this by removing the item from the backpack and adding giving it
/// a [`Position`].
pub fn drop(
    mut commands: Commands,
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    mut players: Query<(&NetworkClient, &Position, &mut Backpack), With<Online>>,
    entities: Query<(Entity, &Details), (With<CanTake>, Without<Position>)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(drop)(( +)(.+))?$").unwrap();
    }

    for message in input.iter() {
        if let Some((client, position, mut backpack)) =
            players.iter_mut().find(|(c, _, _)| c.id == message.id)
        {
            if let Some(captures) = CMD.captures(&message.body.to_lowercase()) {
                debug!("XXXX {:?}", captures);
                match captures.get(4) {
                    Some(name_or_id) => {
                        for i in 0..backpack.0.len() {
                            let entity = backpack.0[i];

                            if let Ok((_, details)) = entities.get(entity) {
                                if details.name.to_lowercase()
                                    == name_or_id.as_str().to_lowercase().trim()
                                    || entity.id().to_string() == name_or_id.as_str()
                                {
                                    commands.entity(entity).insert(Position(position.0));
                                    backpack.0.remove(i);

                                    output.send(NetworkOutput {
                                        id: client.id,
                                        body: format!("You drop the {}.", details.name),
                                    });
                                }

                                break;
                            }
                        }

                        output.send(NetworkOutput {
                            id: client.id,
                            body: "You don't have that.".into(),
                        });
                    }
                    None => {
                        output.send(NetworkOutput {
                            id: client.id,
                            body: "Drop what?".into(),
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
        test::bundles::utils::{item_in_backpack_bundle, player_bundle, ItemBundle, PlayerBundle},
    };

    #[test]
    fn drop() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::drop);

        let item = app
            .world
            .spawn()
            .insert_bundle(item_in_backpack_bundle(ItemBundle {
                name: "Apple".into(),
                ..Default::default()
            }))
            .id();

        let player = app
            .world
            .spawn()
            .insert_bundle(player_bundle(PlayerBundle {
                items: vec![item],
                ..Default::default()
            }))
            .id();

        let player_client_id = app.world.get::<NetworkClient>(player).unwrap().id;

        app.world
            .resource_mut::<Events<NetworkInput>>()
            .send(NetworkInput {
                id: player_client_id,
                body: format!("drop apple"),
                internal: false,
            });

        app.update();

        assert_eq!(app.world.get::<Backpack>(player).unwrap().0.len(), 0);

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You drop the Apple.");
    }

    #[test]
    fn entity_not_found() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::drop);

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
                body: format!("drop apple"),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "You don't have that.");
    }
}
