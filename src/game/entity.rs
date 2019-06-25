use crate::rltk;
use rltk::Color;
use rltk::Rltk;
use rltk::Point;

use super::Map;
use super::Player;
use super::Combat;
use super::Mob;

pub trait BaseEntity {
    fn get_position(&self) -> Point;
    fn get_fg_color(&self) -> Color;
    fn get_glyph(&self) -> u8;

    fn draw_to_map(&self, ctx : &mut Rltk, map : &Map) {
        if map.is_tile_visible(self.get_position()) {
            ctx.con().set(self.get_position(), self.get_fg_color(), Color::black(), self.get_glyph());
        }
    }

    fn as_player(&self) -> Option<&Player> { None }
    fn as_player_mut(&mut self) -> Option<&mut Player> { None }
    fn as_combat(&mut self) -> Option<&mut Combat> { None }
    fn as_mob_mut(&mut self) ->Option<&mut Mob> { None }
    fn plot_visibility(&mut self, map : &Map);
    fn get_tooltip_text(&self) -> String;
    fn blocks_tile(&self) -> bool { false }
    fn can_be_attacked(&self) -> bool { false }
    fn is_dead(&self) -> bool { false }
    fn is_mob(&self) -> bool { false }
    fn get_name(&self) -> String;
}
