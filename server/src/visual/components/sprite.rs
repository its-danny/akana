use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::visual::palette::{hex_to_rgb, rgb_to_color};

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub struct Sprite {
    pub character: String,
    pub color: String,
    pub background: Option<String>,
}

pub trait SpritePaint {
    fn paint(&self) -> Paint<&str>;
}

impl SpritePaint for Sprite {
    fn paint(&self) -> Paint<&str> {
        let painted = Paint::new(self.character.as_str()).fg(rgb_to_color(hex_to_rgb(&self.color)));

        if let Some(background) = &self.background {
            painted.bg(rgb_to_color(hex_to_rgb(background)))
        } else {
            painted
        }
    }
}
