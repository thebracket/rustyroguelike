use crate::rltk;
use rltk::{Color, Rltk, Point};
use super::{Map, Player, Combat, Mob, Item};

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
    fn as_item(&self) -> Option<&Item> { None }
    fn plot_visibility(&mut self, map : &Map);
    fn get_tooltip_text(&self) -> String;
    fn blocks_tile(&self) -> bool { false }
    fn can_be_attacked(&self) -> bool { false }
    fn is_dead(&self) -> bool { false }
    fn is_mob(&self) -> bool { false }
    fn get_name(&self) -> String;
    fn can_pickup(&self) -> bool { false }
    fn is_player(&self) -> bool { false }
}
