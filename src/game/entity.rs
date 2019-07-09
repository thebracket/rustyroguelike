use crate::rltk;
use rltk::{RGB, Rltk, Point, Console};
use super::{Map, Player, Combat, Mob, Item};
extern crate typetag;

#[typetag::serde(tag = "BaseEntity")]
pub trait BaseEntity {
    fn get_position(&self) -> Point;
    fn get_fg_color(&self) -> RGB;
    fn get_glyph(&self) -> u8;

    fn draw_to_map(&self, ctx : &mut Rltk, map : &Map) {
        if map.is_tile_visible(self.get_position()) {
            let pos = self.get_position();     
            ctx.set(pos.x, pos.y, self.get_fg_color(), RGB::named(rltk::BLACK), self.get_glyph());
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

