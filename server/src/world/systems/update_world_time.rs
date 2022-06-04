use bevy::prelude::*;
use chrono::{Local as LocalTime, Timelike};

use crate::world::resources::world_time::{WorldTime, WorldTimeTag};

pub fn update_world_time(mut time: ResMut<WorldTime>) {
    time.time = LocalTime::now();

    time.part = match time.time.hour() {
        // 5am - 6am is Dawn
        5 => WorldTimeTag::Dawn,
        // 6am - 7pm is Day
        6..=19 => WorldTimeTag::Day,
        // 8pm is Dusk
        20 => WorldTimeTag::Dusk,
        // 8pm - 4am is Night
        _ => WorldTimeTag::Night,
    };
}
