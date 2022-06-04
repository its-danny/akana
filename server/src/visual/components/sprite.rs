use bevy::prelude::*;
use yansi::Paint;

#[derive(Component)]
pub struct Sprite {
    pub character: String,
    pub color: String,
}

pub trait SpritePaint {
    fn paint(&self) -> Paint<&str>;
}

impl SpritePaint for Sprite {
    fn paint(&self) -> Paint<&str> {
        match self.color.as_str() {
            "black" => Paint::black(&*self.character),
            "red" => Paint::red(&*self.character),
            "green" => Paint::green(&*self.character),
            "yellow" => Paint::yellow(&*self.character),
            "blue" => Paint::blue(&*self.character),
            "magenta" => Paint::magenta(&*self.character),
            "cyan" => Paint::cyan(&*self.character),
            "white" => Paint::white(&*self.character),
            "black_bold" => Paint::black(&*self.character).bold(),
            "red_bold" => Paint::red(&*self.character).bold(),
            "green_bold" => Paint::green(&*self.character).bold(),
            "yellow_bold" => Paint::yellow(&*self.character).bold(),
            "blue_bold" => Paint::blue(&*self.character).bold(),
            "magenta_bold" => Paint::magenta(&*self.character).bold(),
            "cyan_bold" => Paint::cyan(&*self.character).bold(),
            "white_bold" => Paint::white(&*self.character).bold(),
            _ => Paint::new(&*self.character),
        }
    }
}
