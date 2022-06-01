use bevy::prelude::*;
use yansi::Paint;

#[derive(Component)]
pub(crate) struct Tile {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) sprite: String,
    pub(crate) color: String,
}

pub(crate) trait TilePaint {
    fn paint(&self) -> Paint<&str>;
}

impl TilePaint for Tile {
    fn paint(&self) -> Paint<&str> {
        match self.color.as_str() {
            "black" => Paint::black(&*self.sprite),
            "red" => Paint::red(&*self.sprite),
            "green" => Paint::green(&*self.sprite),
            "yellow" => Paint::yellow(&*self.sprite),
            "blue" => Paint::blue(&*self.sprite),
            "magenta" => Paint::magenta(&*self.sprite),
            "cyan" => Paint::cyan(&*self.sprite),
            "white" => Paint::white(&*self.sprite),
            "black_bold" => Paint::black(&*self.sprite).bold(),
            "red_bold" => Paint::red(&*self.sprite).bold(),
            "green_bold" => Paint::green(&*self.sprite).bold(),
            "yellow_bold" => Paint::yellow(&*self.sprite).bold(),
            "blue_bold" => Paint::blue(&*self.sprite).bold(),
            "magenta_bold" => Paint::magenta(&*self.sprite).bold(),
            "cyan_bold" => Paint::cyan(&*self.sprite).bold(),
            "white_bold" => Paint::white(&*self.sprite).bold(),
            _ => Paint::new(&*self.sprite),
        }
    }
}
