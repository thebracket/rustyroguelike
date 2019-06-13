use crate::rltk;
use rltk::Color;

pub struct Mob {
    pub x : i32,
    pub y : i32,
    pub glyph : u8,
    pub color : Color 
}

impl Mob {
    pub fn new(x:i32, y:i32, glyph:u8, color:Color) -> Mob {
        return Mob{x, y, glyph, color};
    }
}
