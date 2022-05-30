use bevy::prelude::*;
use ldtk_rust::Project;

use crate::{spatial::components::Position, world::components::Tile};

/// Load `assets/world.ldtk` and spawn a whole lot of entitites.
pub(crate) fn setup_world(mut commands: Commands) {
    let project = Project::new("server/assets/world.ldtk");
    let level = project.get_level(0).expect("Could not get level");
    let layers = level
        .layer_instances
        .as_ref()
        .expect("Could not get layer instance");

    for layer in layers.iter().rev() {
        for entity in &layer.entity_instances {
            let x = entity.grid.get(0).expect("Could not get X position");
            let y = entity.grid.get(1).expect("Could not get Y position");

            let name_field = entity
                .field_instances
                .iter()
                .find(|f| f.identifier == "name");

            let name = match name_field {
                Some(name) => name
                    .value
                    .as_ref()
                    .expect("Could not get name value")
                    .to_string(),
                None => "No `name` set.".to_string(),
            };

            debug!("Spawning tile {name} at {x}, {y}, 0");

            commands
                .spawn()
                .insert_bundle((Tile { name }, Position(IVec3::new(*x as i32, *y as i32, 0))));
        }
    }
}
