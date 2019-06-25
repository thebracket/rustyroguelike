use crate::rltk;
use rltk::Color;
use rltk::Rltk;
use rltk::Point;

use super::Map;

pub trait BaseEntity {
    fn get_position(&self) -> Point;
    fn get_fg_color(&self) -> Color;
    fn get_glyph(&self) -> u8;

    fn draw_to_map(&self, ctx : &mut Rltk, map : &Map) {
        if map.is_tile_visible(self.get_position()) {
            ctx.con().set(self.get_position(), self.get_fg_color(), Color::black(), self.get_glyph());
        }
    }
}
