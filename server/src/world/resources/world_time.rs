use chrono::{DateTime, Local as LocalTime};

pub struct WorldTime {
    pub time: DateTime<LocalTime>,
    pub part: WorldTimeTag,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            time: LocalTime::now(),
            part: WorldTimeTag::Dawn,
        }
    }
}

pub enum WorldTimeTag {
    Dawn,
    Day,
    Night,
    Dusk,
}
