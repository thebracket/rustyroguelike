use crate::rltk;
use rltk::Color;

pub struct Mob {
    pub x : i32,
    pub y : i32,
    pub glyph: u8,
    pub fg : Color,
}

impl Mob {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Mob {
        Mob{ x, y, glyph, fg }
    }
}