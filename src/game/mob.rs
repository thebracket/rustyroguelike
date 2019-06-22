use crate::rltk;
use rltk::Color;
use rltk::Point;
use rltk::Algorithm2D;
use rltk::a_star_search;
use super::fighter::Fighter;
use super::Player;
use super::Map;
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

    pub fn turn_tick(&mut self, player: &mut Player, map : &mut Map) -> Vec<String> {
        let can_see_player = self.visible_tiles.contains(&player.position);

        if can_see_player {
            let distance = rltk::distance2d(&mut player.position, &self.position);
            if distance < 1.5 {
                return self.attack_player(player);
            } else {
                self.path_to_player(player, map);
            }
        }
        return Vec::new();
    }

    fn attack_player(&mut self, player: &mut Player) -> Vec<String> {
        let result = attack(self, player);
        return result;
    }

    fn path_to_player(&mut self, player: &mut Player, map : &mut Map) {
        let path = a_star_search(map.point2d_to_index(self.position), map.point2d_to_index(player.position), map);
        if path.success {
            let idx = path.steps[1];
            if !map.is_tile_blocked(idx) {
                let old_idx = (self.position.y * map.width) + self.position.x;
                map.clear_tile_blocked(old_idx);
                map.set_tile_blocked(idx);
                self.position = map.index_to_point2d(idx);
            }
        }
    }
}