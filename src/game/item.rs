use crate::rltk;
use rltk::Color;
use rltk::Point;

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
}