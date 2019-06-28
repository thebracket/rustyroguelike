use crate::rltk;
use rltk::{Color, Point};
use super::{BaseEntity, Map, random_choice};
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ItemType { HealthPotion, ZapScroll, FireballScroll, ConfusionScroll, Sword, Shield }

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ItemSlot { MainHand, OffHand }

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Equippable {
    pub slot : ItemSlot,
    pub power_bonus : i32,
    pub defense_bonus : i32
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Item {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub name : String,
    pub item_type : ItemType,
    pub requires_targeting_mode : bool,
    pub equippable : Option<Equippable>
}

impl Item {
    pub fn new_random(x:i32, y:i32) -> Item {
        let choice = random_choice(vec![
            ("Health".to_string(), 45), 
            ("Zap".to_string(), 10), 
            ("Fireball".to_string(), 10), 
            ("Confusion".to_string(), 10),
            ("Sword".to_string(), 10),
            ("Shield".to_string(), 10),
            ("Dagger".to_string(), 5),
        ]);
        
        if choice == "Health".to_string() { return Item::new_health_potion(x,y); }
        else if choice == "Zap".to_string() { return Item::new_zap_scroll(x,y) }
        else if choice == "Fireball".to_string() { return Item::new_fireball_scroll(x,y) }
        else if choice == "Sword".to_string() { return Item::new_sword(x,y) }
        else if choice == "Shield".to_string() { return Item::new_shield(x,y) }
        else if choice == "Dagger".to_string() { return Item::new_dagger(x,y) }
        else { return Item::new_confusion_scroll(x,y) }
    }

    pub fn new_health_potion(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 173, 
            fg: Color::magenta(), 
            name: "Health Potion".to_string(),
            item_type: ItemType::HealthPotion,
            requires_targeting_mode : false,
            equippable: None
        }
    }

    pub fn new_zap_scroll(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 63, 
            fg: Color::cyan(), 
            name: "Zap Scroll".to_string(),
            item_type: ItemType::ZapScroll,
            requires_targeting_mode : false,
            equippable: None
        }
    }

    pub fn new_fireball_scroll(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 63, 
            fg: Color::orange(), 
            name: "Fireball Scroll".to_string(),
            item_type: ItemType::FireballScroll,
            requires_targeting_mode : true,
            equippable: None
        }
    }

    pub fn new_confusion_scroll(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 63, 
            fg: Color::blue(), 
            name: "Confusion Scroll".to_string(),
            item_type: ItemType::ConfusionScroll,
            requires_targeting_mode : false,
            equippable: None
        }
    }

    pub fn new_sword(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 47, 
            fg: Color::cyan(), 
            name: "Sword".to_string(),
            item_type: ItemType::Sword,
            requires_targeting_mode : false,
            equippable: Some(Equippable{ slot : ItemSlot::MainHand, power_bonus: 1, defense_bonus: 0 })
        }
    }

    pub fn new_dagger(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 47, 
            fg: Color::green(), 
            name: "Dagger".to_string(),
            item_type: ItemType::Sword,
            requires_targeting_mode : false,
            equippable: Some(Equippable{ slot : ItemSlot::MainHand, power_bonus: 2, defense_bonus: 0 })
        }
    }

    pub fn new_shield(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 93, 
            fg: Color::brown(), 
            name: "Shield".to_string(),
            item_type: ItemType::Shield,
            requires_targeting_mode : false,
            equippable: Some(Equippable{ slot : ItemSlot::OffHand, power_bonus: 0, defense_bonus: 1 })
        }
    }
}

#[typetag::serde(name = "BEItem")]
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