use crate::rltk;
use rltk::Color;
use rltk::Console;

use super::Player;
use super::Mob;
use super::Map;

pub trait Renderable {
    fn draw(&self, console : &mut Console, map : &Map);
}

impl Renderable for Player {
    fn draw(&self, console : &mut Console, map : &Map) {
        if map.is_tile_visible(&self.position) {
            let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
            console.set(&self.position, fg, Color::black(), self.glyph);
        }
    }
}

impl Renderable for Mob {
    fn draw(&self, console : &mut Console, map : &Map) {
        if map.is_tile_visible(&self.position) {
            let fg = Color::new(self.fg.r, self.fg.g, self.fg.b);
            console.set(&self.position, fg, Color::black(), self.glyph);
        }
    }
}
