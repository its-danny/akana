use bevy::prelude::*;

pub(crate) struct NewPlayerSpawn(pub(crate) IVec3);

impl Default for NewPlayerSpawn {
    fn default() -> Self {
        Self(IVec3::new(0, 0, 0))
    }
}
