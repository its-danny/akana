use bevy::{prelude::*, utils::HashMap};
use inflector::Inflector;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    items::components::{backpack::Backpack, can_take::CanTake},
    network::events::{NetworkInput, NetworkOutput},
    player::components::{client::NetworkClient, online::Online},
    spatial::components::position::Position,
    visual::components::{
        details::Details,
        sprite::{Sprite, SpritePaint},
    },
};

pub fn backpack(
    mut input: EventReader<NetworkInput>,
    mut output: EventWriter<NetworkOutput>,
    players: Query<(&NetworkClient, &Backpack), With<Online>>,
    entities: Query<(Entity, &Details, &Sprite), (With<CanTake>, Without<Position>)>,
) {
    lazy_static! {
        static ref CMD: Regex = Regex::new("^(backpack|pack|bp|inventory|inv|i)$").unwrap();
    }

    for message in input.iter() {
        if let Some((client, backpack)) = players.iter().find(|(c, _)| c.id == message.id) {
            if CMD.is_match(&message.body.to_lowercase()) {
                let mut items = Vec::new();
                let mut counted: HashMap<String, (&Details, &Sprite, i32)> = HashMap::new();

                for entity in &backpack.0 {
                    if let Some((_, details, sprite)) =
                        entities.iter().find(|(e, _, _)| e == entity)
                    {
                        if counted.contains_key(&details.name) {
                            counted.insert(
                                details.name.clone(),
                                (
                                    details,
                                    sprite,
                                    counted
                                        .get(&details.name)
                                        .unwrap_or(&(details, sprite, 0))
                                        .2
                                        + 1,
                                ),
                            );
                        } else {
                            counted.insert(details.name.clone(), (details, sprite, 1));
                        }
                    }
                }

                for (_, (details, sprite, count)) in counted.iter() {
                    items.push(format!(
                        "{} {} {}",
                        sprite.paint(),
                        count,
                        if *count > 1 {
                            details.name.to_plural()
                        } else {
                            details.name.clone()
                        }
                    ));
                }

                if items.is_empty() {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: "Your backpack is empty.".into(),
                    });
                } else {
                    output.send(NetworkOutput {
                        id: client.id,
                        body: format!("Your backpack contains:\r\n{}", items.join("\r\n")),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, prelude::*};
    use yansi::Paint;

    use crate::{
        network::events::{NetworkInput, NetworkOutput},
        player::components::client::NetworkClient,
        test::bundles::utils::{item_in_backpack_bundle, player_bundle, ItemBundle, PlayerBundle},
    };

    #[test]
    fn backpack() {
        Paint::disable();

        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::backpack);

        let item = app
            .world
            .spawn()
            .insert_bundle(item_in_backpack_bundle(ItemBundle {
                name: "Apple".into(),
                character: "o".into(),
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
                body: format!("backpack"),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(
            output.body,
            "Your backpack contains:\r\no 1 Apple".to_string()
        );
    }

    #[test]
    fn empty() {
        let mut app = App::new();

        app.add_event::<NetworkInput>();
        app.add_event::<NetworkOutput>();
        app.add_system(super::backpack);

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
                body: format!("backpack"),
                internal: false,
            });

        app.update();

        let output_events = app.world.resource::<Events<NetworkOutput>>();
        let mut output_reader = output_events.get_reader();
        let output = output_reader.iter(output_events).next().unwrap();

        assert_eq!(output.id, player_client_id);
        assert_eq!(output.body, "Your backpack is empty.");
    }
}
