use crate::rltk;
use rltk::Color;
use rltk::Console;

mod tiletype;
pub use tiletype::TileType;

mod player;
pub use player::Player;

mod mob;
pub use mob::Mob;

mod rect;
pub use rect::Rect;

mod renderable;
pub use renderable::Renderable;

mod map;
pub use map::Map;

extern crate rand;

pub struct State {
    pub map : Map,
    pub player : Player,
    pub mobs : Vec<Mob>
}

impl State {
    pub fn new() -> State {
        let mut map = Map::new();
        let rooms = map.random_rooms_tut3();

        let (player_x, player_y) = rooms[0].center();

        let mut mobs : Vec<Mob> = Vec::new();
        for i in 1 .. rooms.len() {
            let (room_x, room_y) = rooms[i].center();
            let mob = Mob::new(room_x, room_y, 98, Color::red());
            mobs.push(mob);
        }

        return State{ map: map, player: Player::new(player_x, player_y, 64, Color::yellow()), mobs: mobs };
    }

    // Utility function: find the index of a tile at x/y
    fn tile_idx(&self, x:i32, y:i32) -> Option<usize> {
        if self.valid_tile(x, y) {
            return Some(((y*80)+x) as usize);
        } else {
            return None;
        }
    }

    // Utility function: bounds checking
    fn valid_tile(&self, x:i32, y:i32) -> bool {
        return x > 0 && x < 79 && y > 0 && y < 49;
    }

    // Utility function: is a tile walkable
    fn is_walkable(&mut self, x:i32, y:i32) -> bool {
        let idx = self.tile_idx(x, y);
        match idx {
            Some(idx) => {
                match self.map.tiles[idx] {
                    TileType::Floor => { return true }
                    TileType::Wall => { return false }
                }
            }

            None => {
                return false;
            }
        }
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.x + delta_x;
        let new_y = self.player.y + delta_y;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.is_walkable(new_x, new_y) {
            self.player.x = new_x;
            self.player.y = new_y;
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.map.draw(console);
        self.player.draw(console);
        for mob in self.mobs.iter() {
            mob.draw(console);
        }

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }

                328 => { self.move_player(0, -1) }
                336 => { self.move_player(0, 1) }
                331 => { self.move_player(-1, 0) }
                333 => { self.move_player(1, 0) }

                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }                
                }
            }
            None => {}
        }
    }
}