use crate::rltk;
use rltk::Color;
use rltk::Console;

use super::Player;
use super::Mob;

pub trait Renderable {
    fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Self;
    fn draw(&self, console : &mut Console);
}

impl Renderable for Player {
    fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Player {
        Player{ x, y, glyph, fg }
    }

    fn draw(&self, console : &mut Console) {
        let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
        console.set(self.x as u32, self.y as u32, fg, Color::black(), self.glyph);
    }
}

impl Renderable for Mob {
    fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Mob {
        Mob{ x, y, glyph, fg }
    }

    fn draw(&self, console : &mut Console) {
        let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
        console.set(self.x as u32, self.y as u32, fg, Color::black(), self.glyph);
    }
}