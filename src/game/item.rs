use crate::rltk;
use rltk::{Color, Point};
use super::{BaseEntity, Map};

#[derive(PartialEq, Clone, Copy)]
pub enum ItemType { HealthPotion, ZapScroll, FireballScroll }

#[derive(PartialEq, Clone)]
pub struct Item {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub name : String,
    pub item_type : ItemType,
    pub requires_targeting_mode : bool
}

impl Item {
    pub fn new_random(x:i32, y:i32, n:i32) -> Item {
        match n {
            1 => { return Item::new_zap_scroll(x,y) }
            2 => { return Item::new_fireball_scroll(x,y) }
            _ => { return Item::new_health_potion(x,y) }
        }
    }

    pub fn new_health_potion(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 173, 
            fg: Color::magenta(), 
            name: "Health Potion".to_string(),
            item_type: ItemType::HealthPotion,
            requires_targeting_mode : false
        }
    }

    pub fn new_zap_scroll(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 63, 
            fg: Color::cyan(), 
            name: "Zap Scroll".to_string(),
            item_type: ItemType::ZapScroll,
            requires_targeting_mode : false
        }
    }

    pub fn new_fireball_scroll(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 63, 
            fg: Color::orange(), 
            name: "Fireball Scroll".to_string(),
            item_type: ItemType::FireballScroll,
            requires_targeting_mode : true
        }
    }
}

impl BaseEntity for Item {
    fn get_position(&self) -> Point { self.position }
    fn get_fg_color(&self) -> Color { self.fg }
    fn get_glyph(&self) -> u8 { self.glyph }
    fn plot_visibility(&mut self, _map : &Map) {}
    fn get_tooltip_text(&self) -> String { format!("Item: {}", self.name) }
    fn get_name(&self) -> String { self.name.clone() }
    fn can_pickup(&self) -> bool { true }
    fn as_item(&self) -> Option<&Item> { Some(self) }
}