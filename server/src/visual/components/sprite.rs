use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};
use yansi::{Color, Paint};

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub struct Sprite {
    pub character: String,
    pub color: [u8; 3],
    pub background: Option<[u8; 3]>,
}

pub trait SpritePaint {
    fn paint(&self, background: Option<[u8; 3]>) -> Paint<&str>;
}

impl SpritePaint for Sprite {
    fn paint(&self, background: Option<[u8; 3]>) -> Paint<&str> {
        let background = background.unwrap_or_else(|| self.background.unwrap_or([22, 22, 22]));

        Paint::new(self.character.as_str())
            .fg(Color::RGB(self.color[0], self.color[1], self.color[2]))
            .bg(Color::RGB(background[0], background[1], background[2]))
    }
}
