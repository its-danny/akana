use bevy::prelude::*;
use chrono::{Local as LocalTime, Timelike};
use ldtk_rust::Project;

use crate::{spatial::components::Position, world::components::Tile};

use super::resources::{NewPlayerSpawn, WorldTime, WorldTimePart};

/// Load `assets/world.ldtk` and spawn a whole lot of entities.
pub(crate) fn setup_world(mut commands: Commands, mut new_player_spawn: ResMut<NewPlayerSpawn>) {
    let project = Project::new("server/assets/world.ldtk");
    let level = project.get_level(0).unwrap();
    let layers = level.layer_instances.as_ref().unwrap();

    for layer in layers.iter().rev() {
        if layer.identifier == "Tiles" {
            for entity in &layer.entity_instances {
                let fields: [&str; 4] = ["name", "description", "sprite", "color"].map(|field| {
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

                // An entity in LDTK can span multiple grid points.
                // Because of that, we want to create a tile for each grid
                // point it covers.
                for x in 0..(entity.width / layer.grid_size) {
                    for y in 0..(entity.height / layer.grid_size) {
                        let x: i32 = (x + entity.grid.get(0).unwrap()).try_into().unwrap();
                        let y: i32 = (y + entity.grid.get(1).unwrap()).try_into().unwrap();

                        commands.spawn().insert_bundle((
                            Tile {
                                name: fields[0].to_string(),
                                description: fields[1].to_string(),
                                sprite: fields[2].to_string(),
                                color: fields[3].to_string(),
                            },
                            Position(IVec3::new(x, y, 0)),
                        ));
                    }
                }
            }
        }

        if layer.identifier == "New_Player_Spawn" {
            let entity = &layer.entity_instances.get(0).unwrap();

            let x = *entity.grid.get(0).unwrap() as i32;
            let y = *entity.grid.get(1).unwrap() as i32;

            new_player_spawn.0 = IVec3::new(x, y, 0);
        }
    }
}

pub(crate) fn update_world_time(mut time: ResMut<WorldTime>) {
    time.time = LocalTime::now();

    time.part = match time.time.hour() {
        // 5am - 6am is Dawn
        5 => WorldTimePart::Dawn,
        // 6am - 7pm is Day
        6..=19 => WorldTimePart::Day,
        // 8pm is Dusk
        20 => WorldTimePart::Dusk,
        // 8pm - 4am is Night
        _ => WorldTimePart::Night,
    };
}
