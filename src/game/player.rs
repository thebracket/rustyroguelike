use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;
use super::Inventory;
use super::Combat;

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