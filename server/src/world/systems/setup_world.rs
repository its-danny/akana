use bevy::prelude::*;
use ldtk_rust::Project;

use crate::{
    spatial::components::{collider::Collider, door::Door, position::Position},
    visual::components::sprite::Sprite,
    world::{components::tile::Tile, resources::new_player_spawn::NewPlayerSpawn},
};

/// Load `assets/world.ldtk` and spawn a whole lot of entities.
pub fn setup_world(mut commands: Commands, mut new_player_spawn: ResMut<NewPlayerSpawn>) {
    let project = Project::new("server/assets/world.ldtk");
    let level = project.get_level(0).unwrap();
    let layers = level.layer_instances.as_ref().unwrap();

    for layer in layers.iter().rev() {
        if layer.identifier == "Tiles" {
            for entity in &layer.entity_instances {
                let str_fields: [&str; 4] =
                    ["name", "description", "sprite", "color"].map(|field| {
                        entity
                            .field_instances
                            .iter()
                            .find(|f| f.identifier == field)
                            .unwrap_or_else(|| panic!("Could not find `{field}` field"))
                            .value
                            .as_ref()
                            .unwrap_or_else(|| panic!("Could not get `{field}` value"))
                            .as_str()
                            .unwrap_or_else(|| panic!("Could not get `{field}` as str"))
                    });

                let bool_fields: [bool; 1] = ["collision"].map(|field| {
                    entity
                        .field_instances
                        .iter()
                        .find(|f| f.identifier == field)
                        .unwrap_or_else(|| panic!("Could not find `{field}` field"))
                        .value
                        .as_ref()
                        .unwrap_or_else(|| panic!("Could not get `{field}` value"))
                        .as_bool()
                        .unwrap_or_else(|| panic!("Could not get `{field}` as bool"))
                });

                // An entity in LDTK can span multiple grid points.
                // Because of that, we want to create a tile for each grid
                // point it covers.
                for x in 0..(entity.width / layer.grid_size) {
                    for y in 0..(entity.height / layer.grid_size) {
                        let x: i32 = (x + entity.grid.get(0).unwrap()).try_into().unwrap();
                        let y: i32 = (y + entity.grid.get(1).unwrap()).try_into().unwrap();

                        let mut entity = commands.spawn();

                        entity.insert_bundle((
                            Tile {
                                name: str_fields[0].to_string(),
                                description: str_fields[1].to_string(),
                            },
                            Sprite {
                                character: str_fields[2].to_string(),
                                color: str_fields[3].to_string(),
                            },
                            Position(IVec2::new(x, y)),
                        ));

                        if bool_fields[0] {
                            entity.insert(Collider);
                        }
                    }
                }
            }
        }

        if layer.identifier == "Entities" {
            for entity in &layer.entity_instances {
                let x = *entity.grid.get(0).unwrap() as i32;
                let y = *entity.grid.get(1).unwrap() as i32;

                if entity.identifier == "Door" {
                    let bool_fields: [bool; 1] = ["horizontal"].map(|field| {
                        entity
                            .field_instances
                            .iter()
                            .find(|f| f.identifier == field)
                            .unwrap_or_else(|| panic!("Could not find `{field}` field"))
                            .value
                            .as_ref()
                            .unwrap_or_else(|| panic!("Could not get `{field}` value"))
                            .as_bool()
                            .unwrap_or_else(|| panic!("Could not get `{field}` as bool"))
                    });

                    let facing = if bool_fields[0] {
                        "horizontal"
                    } else {
                        "vertical"
                    };

                    let mut entity = commands.spawn();

                    entity.insert_bundle((
                        Door {
                            facing: facing.to_string(),
                        },
                        Position(IVec2::new(x, y)),
                        Collider,
                        Sprite {
                            character: if facing == "horizontal" { "|" } else { "_" }.to_string(),
                            color: "white".to_string(),
                        },
                    ));
                }
            }
        }

        if layer.identifier == "Spawn" {
            let entity = &layer.entity_instances.get(0).unwrap();

            let x = *entity.grid.get(0).unwrap() as i32;
            let y = *entity.grid.get(1).unwrap() as i32;

            new_player_spawn.0 = IVec2::new(x, y);
        }
    }
}
