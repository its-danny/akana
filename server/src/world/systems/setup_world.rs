use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;
use ldtk_rust::Project;

use crate::{
    spatial::components::position::Position, world::resources::new_player_spawn::NewPlayerSpawn,
};

/// Load `assets/world.ldtk` and spawn a whole lot of entities.
pub fn setup_world(
    mut commands: Commands,
    prototypes: Res<ProtoData>,
    asset_server: Res<AssetServer>,
    mut new_player_spawn: ResMut<NewPlayerSpawn>,
) {
    let project = Project::new("server/assets/world.ldtk");
    let level = project.get_level(0).unwrap();
    let layers = level.layer_instances.as_ref().unwrap();

    debug!(
        "Known Prototypes: {:?}",
        prototypes
            .iter()
            .map(|p| p.name())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    for layer in layers.iter().rev() {
        for entity in &layer.entity_instances {
            match layer.identifier.as_str() {
                "Spawn" => {
                    let x = *entity.grid.get(0).unwrap() as i32;
                    let y = *entity.grid.get(1).unwrap() as i32;

                    new_player_spawn.0 = IVec2::new(x, y);
                }
                _ => {
                    let key = entity
                        .field_instances
                        .get(0)
                        .unwrap_or_else(|| panic!("Could not get `prototype` field"))
                        .value
                        .as_ref()
                        .unwrap_or_else(|| panic!("Could not get `prototype` value"))
                        .as_str()
                        .unwrap_or_else(|| panic!("Could not get `prototype` as bool"));

                    let prototype = prototypes
                        .get_prototype(key)
                        .unwrap_or_else(|| panic!("Could not find `{key}` prototype"));

                    for x in 0..(entity.width / layer.grid_size) {
                        for y in 0..(entity.height / layer.grid_size) {
                            let x: i32 = (x + entity.grid.get(0).unwrap()).try_into().unwrap();
                            let y: i32 = (y + entity.grid.get(1).unwrap()).try_into().unwrap();

                            prototype
                                .spawn(&mut commands, &prototypes, &asset_server)
                                .insert(Position(IVec2::new(x, y)));
                        }
                    }
                }
            }
        }
    }
}
