use crate::rltk;
use rltk::Color;
use rltk::Point;
use super::fighter::Fighter;
use super::Player;
//use super::fighter::attack;

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
            if distance < 1.5 {
                self.attack_player(player);
            } else {
                self.path_to_player(player, blocked);
            }
        }
    }

    fn attack_player(&mut self, _player: &mut Player) {
        /*let result = attack(self, player);
        for s in result.iter() {
            println!("{}", s);
        }*/
        println!("{} calls you terrible names.", self.name);
    }

    fn path_to_player(&mut self, player: &mut Player, blocked : &mut Vec<bool>) {
        let is_blocked = |idx:&Point| -> bool { return blocked[((idx.y * 80)+idx.x) as usize]; };
        let mut starts : Vec<Point> = Vec::new();
        starts.push(player.position.clone());
        let dmap = rltk::DijkstraMap::new(80, 50, &starts, &is_blocked, 8);
        let dest = dmap.find_lowest_exit(self.position);

        match dest {
            None => {}
            Some(d) => {
                let idx = ((d.y * 80) + d.x) as usize;
                if !blocked[idx] {
                    let old_idx = ((self.position.y * 80) + self.position.x) as usize;
                    blocked[old_idx] = false;
                    blocked[idx] = true;
                    self.position.x = d.x;
                    self.position.y = d.y;
                }

            }
        }
    }
}