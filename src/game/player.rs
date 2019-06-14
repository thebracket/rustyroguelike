use crate::rltk;
use rltk::Color;

pub struct Player {
    pub x : i32,
    pub y : i32,
    pub glyph: u8,
    pub fg : Color,
}

impl Player {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Player {
        Player{ x, y, glyph, fg }
    }
}