use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;
use super::Inventory;
use super::BaseEntity;
use super::Combat;
use rltk::field_of_view;
use super::Map;
use super::Item;
use super::ItemType;

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

        let item_type = self.inventory.items[item_index as usize].item_type;
        match item_type {
            ItemType::HealthPotion => {
                if self.fighter.hp == self.fighter.max_hp {
                    result.push("You are already at maximum health.".to_string());
                    return result;
                }

                self.fighter.hp = self.fighter.max_hp; // Cheezed due to confusion over borrowing
                result.push("You are healed!".to_string());
                self.inventory.items.remove(item_index as usize);
            }
            _ => {}
        }

        return result;
    }

    pub fn remove_item_from_inventory(&mut self, item_index: i32) -> Item {
        let item_copy = self.inventory.items[item_index as usize].clone();
        self.inventory.items.remove(item_index as usize);
        return item_copy;
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
    fn get_tooltip_text(&self) -> String { "It's you!".to_string() }
    fn get_name(&self) -> String { "Player".to_string() }
}