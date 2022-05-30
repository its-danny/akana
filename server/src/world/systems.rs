use bevy::prelude::*;
use ldtk_rust::Project;

use crate::{spatial::components::Position, world::components::Tile};

use super::resources::NewPlayerSpawn;

/// Load `assets/world.ldtk` and spawn a whole lot of entitites.
///
/// We `unwrap` all over the place because we don't really care if this panics,
/// big trouble if the world can't load in.
pub(crate) fn setup_world(mut commands: Commands, mut new_player_spawn: ResMut<NewPlayerSpawn>) {
    let project = Project::new("server/assets/world.ldtk");
    let level = project.get_level(0).unwrap();
    let layers = level.layer_instances.as_ref().unwrap();

    for layer in layers.iter().rev() {
        if layer.identifier == "Tiles" {
            for entity in &layer.entity_instances {
                let x = *entity.grid.get(0).unwrap() as i32;
                let y = *entity.grid.get(1).unwrap() as i32;

                let name = entity
                    .field_instances
                    .get(0)
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .to_string();

                commands
                    .spawn()
                    .insert_bundle((Tile { name }, Position(IVec3::new(x, y, 0))));
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
