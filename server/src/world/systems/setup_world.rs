use bevy::prelude::*;
use ldtk_rust::{EntityInstance, LayerInstance, Project};

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
        for entity in &layer.entity_instances {
            match layer.identifier.as_str() {
                "Spawn" => set_player_spawn(entity, &mut new_player_spawn),
                "Tiles" => spawn_tile(entity, layer, &mut commands),
                "Entities" => {
                    if let "Door" = entity.identifier.as_str() {
                        spawn_door(entity, &mut commands)
                    }
                }
                _ => (),
            }
        }
    }
}

fn set_player_spawn(entity: &EntityInstance, spawn: &mut ResMut<'_, NewPlayerSpawn>) {
    let x = *entity.grid.get(0).unwrap() as i32;
    let y = *entity.grid.get(1).unwrap() as i32;

    spawn.0 = IVec2::new(x, y);
}

fn spawn_tile(entity: &EntityInstance, layer: &LayerInstance, commands: &mut Commands) {
    let str_fields: [&str; 4] = ["name", "description", "sprite", "color"].map(|field| {
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

fn spawn_door(entity: &EntityInstance, commands: &mut Commands) {
    let x = *entity.grid.get(0).unwrap() as i32;
    let y = *entity.grid.get(1).unwrap() as i32;

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
