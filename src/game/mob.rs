use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;

pub struct Mob {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub visible_tiles : Vec<Point>,
    pub name : String,
    pub fighter : Fighter
}

impl Mob {
    pub fn new_random(x:i32, y:i32, n:i32) -> Mob {
        match n {
            1 => { return Mob::new_wight(x,y) }
            2 => { return Mob::new_iter(x,y) }
            _ => { return Mob::new_hound(x, y) }
        }
    }

    fn new_wight(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 98, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Borrow Wight".to_string(),
            fighter: Fighter::new(8, 1, 1)
        }
    }

    fn new_hound(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 109, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Mut Hound".to_string(),
            fighter: Fighter::new(8, 1, 1)
        }
    }

    fn new_iter(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 105, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Itereater Beast".to_string(),
            fighter: Fighter::new(8, 1, 1)
        }
    }
}