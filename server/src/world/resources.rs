use bevy::prelude::*;
use chrono::{DateTime, Local as LocalTime};

pub(crate) struct NewPlayerSpawn(pub(crate) IVec3);

impl Default for NewPlayerSpawn {
    fn default() -> Self {
        Self(IVec3::new(0, 0, 0))
    }
}

pub(crate) struct WorldTime {
    pub(crate) time: DateTime<LocalTime>,
    pub(crate) part: WorldTimePart,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            time: LocalTime::now(),
            part: WorldTimePart::Dawn,
        }
    }
}

pub(crate) enum WorldTimePart {
    Dawn,
    Day,
    Night,
    Dusk,
}
