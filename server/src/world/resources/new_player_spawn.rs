use bevy::prelude::*;

pub struct NewPlayerSpawn(pub IVec2);

impl Default for NewPlayerSpawn {
    fn default() -> Self {
        Self(IVec2::new(0, 0))
    }
}
