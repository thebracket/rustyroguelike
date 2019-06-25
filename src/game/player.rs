use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;
use super::Inventory;
use super::BaseEntity;
use super::Combat;
use rltk::field_of_view;
use super::Map;

pub struct Player {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub visible_tiles : Vec<Point>,
    pub fighter : Fighter,
    pub inventory : Inventory
}

impl Player {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Player {
        Player{ 
            position: Point::new(x, y), 
            glyph, fg, 
            visible_tiles: Vec::new(), 
            fighter: Fighter::new(8, 0, 1),
            inventory: Inventory::new(4)
        }
    }

    pub fn use_item(&mut self, item_index : i32) -> Vec<String> {
        let mut result = Vec::new();

        self.fighter.hp = self.fighter.max_hp; // Cheezed due to confusion over borrowing
        let r = self.inventory.items[item_index as usize].consume();
        for tmp in r { result.push(tmp); }
        self.inventory.items.remove(item_index as usize);

        return result;
    }
}

impl BaseEntity for Player {
    fn get_position(&self) -> Point { self.position }
    fn get_fg_color(&self) -> Color { self.fg }
    fn get_glyph(&self) -> u8 { self.glyph }
    fn as_player(&self) -> Option<&Player> { Some(self) }
    fn as_player_mut(&mut self) -> Option<&mut Player> { Some(self) }
    fn as_combat(&mut self) -> Option<&mut Combat> { Some(self) }
    fn plot_visibility(&mut self, map : &Map) {
        self.visible_tiles = field_of_view(self.get_position(), 6, map);
    }
}