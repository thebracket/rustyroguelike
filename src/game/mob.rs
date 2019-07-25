use crate::rltk;
use rltk::{RGB, Point, Algorithm2D, a_star_search, field_of_view};
use super::{fighter::Fighter, Map, Combat, BaseEntity, State, attack, random_choice, Particle};
use rand::Rng;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Mob {
    pub position : Point,
    pub glyph: u8,
    pub fg : RGB,
    pub visible_tiles : Vec<Point>,
    pub name : String,
    pub fighter : Fighter,
    pub confused: Option<i32>
}

impl Mob {
    pub fn new_random(x:i32, y:i32) -> Mob {
        let choice = random_choice(vec![("Wight".to_string(), 10), ("Hound".to_string(), 45), ("Itereater".to_string(), 45)]);
        if choice == "Wight".to_string() { return Mob::new_wight(x, y); }
        else if choice == "Hound".to_string() { return Mob::new_hound(x, y); }
        else { return Mob::new_iter(x, y); }
    }

    fn new_wight(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 38, 
            fg: RGB::named(rltk::RED), 
            visible_tiles: Vec::new(), 
            name: "Borrow Wight".to_string(),
            fighter: Fighter::new(2, 0, 1, 60),
            confused: None
        }
    }

    fn new_hound(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 109, 
            fg: RGB::named(rltk::RED), 
            visible_tiles: Vec::new(), 
            name: "Mut Hound".to_string(),
            fighter: Fighter::new(1, 0, 1, 30),
            confused: None
        }
    }

    fn new_iter(x:i32, y:i32) -> Mob {
        Mob{ 
            position: Point::new(x, y), 
            glyph: 105, 
            fg: RGB::named(rltk::RED), 
            visible_tiles: Vec::new(), 
            name: "Itereater Beast".to_string(),
            fighter: Fighter::new(1, 0, 1, 30),
            confused: None
        }
    }

    pub fn turn_tick(&mut self, player_pos : Point, map : &mut Map) -> bool {
        match self.confused {
            Some(turns) => {
                let new_turns = turns-1;
                if new_turns == 0 {
                    self.confused = None;
                } else {
                    self.confused = Some(new_turns);
                }

                let mut rng = rand::thread_rng();
                let delta_x = rng.gen_range(0, 3)-1;
                let delta_y = rng.gen_range(0, 3)-1;
                let new_loc = Point::new(self.position.x + delta_x, self.position.y + delta_y);
                if map.is_walkable(new_loc.x, new_loc.y) && !map.is_tile_blocked(map.point2d_to_index(new_loc)) {
                    self.position = new_loc;
                }

                return false;
            }
            None => {}
        }

        let can_see_player = self.visible_tiles.contains(&player_pos);

        if can_see_player {
            let distance = rltk::distance2d(rltk::DistanceAlg::Pythagoras, player_pos, self.position);
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

#[typetag::serde(name = "BEMob")]
impl BaseEntity for Mob {
    fn get_position(&self) -> Point { self.position }
    fn get_fg_color(&self) -> RGB { self.fg }
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

pub fn mob_tick(gs : &mut State) {
    // Build the master map of unavailable tiles
    gs.map.refresh_blocked();
    for e in gs.entities.iter() {
        if e.blocks_tile() {
            let pos = e.get_position();
            gs.map.set_tile_blocked(gs.map.point2d_to_index(pos));
        }
    }

    let mut i : usize = 0;
    let mut active_mobs : Vec<usize> = Vec::new();
    for e in gs.entities.iter_mut() {
        if e.is_mob() { active_mobs.push(i); }
        i += 1;
    }

    let ppos = gs.player().position;
    let mut attacking_mobs : Vec<usize> = Vec::new();

    for id in active_mobs {
        let mob = gs.entities[id].as_mob_mut().unwrap();
        if mob.turn_tick(ppos, &mut gs.map) {
            attacking_mobs.push(id);
        }
    }

    let mut tmp : Vec<String> = Vec::new();
    for id in attacking_mobs {
        let attacker_name = gs.entities[id].get_name();
        let attacker_power = gs.entities[id].as_combat().unwrap().get_power();
        gs.vfx.push(Particle::new(gs.player().get_position(), RGB::named(rltk::RED), RGB::named(rltk::BLACK), 176, 200.0));
        let (_xp, result) = attack(attacker_name, attacker_power, gs.player_as_combat());
        for r in result {
            tmp.push(r);
        }
    }
    for s in tmp {
        gs.add_log_entry(s);
    }
}
