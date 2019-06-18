use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;
use super::Player;
use super::fighter::attack;

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
            fighter: Fighter::new(8, 0, 1)
        }
    }

    fn new_hound(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 109, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Mut Hound".to_string(),
            fighter: Fighter::new(8, 0, 1)
        }
    }

    fn new_iter(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 105, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Itereater Beast".to_string(),
            fighter: Fighter::new(8, 0, 1)
        }
    }

    pub fn turn_tick(&mut self, player: &mut Player, blocked : &mut Vec<bool>) {
        let can_see_player = self.visible_tiles.contains(&player.position);

        if can_see_player {
            let distance = rltk::distance2d(&mut player.position, &self.position);
            if distance < 1.2 {
                self.attack_player(player);
            } else {
                self.path_to_player(player, blocked);
            }
        }
    }

    fn attack_player(&mut self, player: &mut Player) {
        /*let result = attack(self, player);
        for s in result.iter() {
            println!("{}", s);
        }*/
        println!("{} calls you terrible names.", self.name);
    }

    fn path_to_player(&mut self, player: &mut Player, blocked : &mut Vec<bool>) {
        if self.position.x > player.position.x { self.move_mob(-1, 0, blocked); }
        if self.position.x < player.position.x { self.move_mob(1, 0, blocked); }
        if self.position.y > player.position.y { self.move_mob(0, -1, blocked); }
        if self.position.y < player.position.y { self.move_mob(0, 1, blocked); }
    }

    fn move_mob(&mut self, delta_x : i32, delta_y : i32, blocked : &mut Vec<bool>) {
        let destination_x = self.position.x + delta_x;
        let destination_y = self.position.y + delta_y;
        let idx = ((destination_y * 80) + destination_x) as usize;
    
        if !blocked[idx] {
            let old_idx = ((self.position.y * 80) + self.position.x) as usize;
            blocked[old_idx] = false;
            self.position.x = destination_x;
            self.position.y = destination_y;
            blocked[idx] = true;
        }
    }
}