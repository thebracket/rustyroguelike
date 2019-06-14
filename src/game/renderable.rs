use crate::rltk;
use rltk::Color;
use rltk::Console;

use super::Player;
use super::Mob;

pub trait Renderable {
    fn draw(&self, console : &mut Console);
}

impl Renderable for Player {
    fn draw(&self, console : &mut Console) {
        let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
        console.set(self.position.x as u32, self.position.y as u32, fg, Color::black(), self.glyph);
    }
}

impl Renderable for Mob {
    fn draw(&self, console : &mut Console) {
        let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
        console.set(self.position.x as u32, self.position.y as u32, fg, Color::black(), self.glyph);
    }
}
