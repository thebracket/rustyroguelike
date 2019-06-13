use crate::rltk;
use rltk::Color;

pub struct Player {
    pub x : i32,
    pub y : i32,
    pub glyph: u8,
    pub fg : Color,
}
