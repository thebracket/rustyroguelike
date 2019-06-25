use crate::rltk;
use rltk::Color;
use rltk::Point;
use rltk::Algorithm2D;
use rltk::a_star_search;
use rltk::field_of_view;
use super::fighter::Fighter;
use super::Player;
use super::Map;
use super::fighter::attack;
use super::Combat;
use super::BaseEntity;

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
            glyph: 38, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Borrow Wight".to_string(),
            fighter: Fighter::new(2, 0, 1)
        }
    }

    fn new_hound(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 109, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Mut Hound".to_string(),
            fighter: Fighter::new(1, 0, 1)
        }
    }

    fn new_iter(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 105, 
            fg: Color::red(), 
            visible_tiles: Vec::new(), 
            name: "Itereater Beast".to_string(),
            fighter: Fighter::new(1, 0, 1)
        }
    }

    pub fn turn_tick(&mut self, player_pos : Point, map : &mut Map) -> bool {
        let can_see_player = self.visible_tiles.contains(&player_pos);

        if can_see_player {
            let distance = rltk::distance2d(player_pos, self.position);
            if distance < 1.5 {
                return true;
            } else {
                self.path_to_player(player_pos, map);
            }
        }
        return false;
    }

    fn path_to_player(&mut self, player_pos : Point, map : &mut Map) {
        let path = a_star_search(map.point2d_to_index(self.position), map.point2d_to_index(player_pos), map);
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

impl BaseEntity for Mob {
    fn get_position(&self) -> Point { self.position }
    fn get_fg_color(&self) -> Color { self.fg }
    fn get_glyph(&self) -> u8 { self.glyph }
    fn as_combat(&mut self) -> Option<&mut Combat> { Some(self) }
    fn plot_visibility(&mut self, map : &Map) {
        self.visible_tiles = field_of_view(self.get_position(), 6, map);
    }
    fn get_tooltip_text(&self) -> String { format!("Enemy: {}", self.name) }
    fn blocks_tile(&self) -> bool { true }
    fn can_be_attacked(&self) -> bool { true }
    fn is_dead(&self) -> bool { self.fighter.dead }
    fn is_mob(&self) -> bool { true }
    fn as_mob_mut(&mut self) ->Option<&mut Mob> { Some(self) }
    fn get_name(&self) -> String { self.name.to_string() }
}