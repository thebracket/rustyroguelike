use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::BaseEntity;
use super::Map;

#[derive(PartialEq, Clone)]
pub struct Item {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub name : String
}

impl Item {
    pub fn new_health_potion(x:i32, y:i32) -> Item {
        Item{ 
            position: Point::new(x, y), 
            glyph: 173, 
            fg: Color::magenta(), 
            name: "Health Potion".to_string()
        }
    }

    pub fn consume(&self) -> Vec<String> {
        let mut result = Vec::new();

        result.push(format!("You drink the {}. You are healed!", self.name));
        //target.heal_damage(100);

        return result;
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