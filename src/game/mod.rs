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

mod visibility;
pub use visibility::Visibility;

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

        let mut player = Player::new(player_x, player_y, 64, Color::yellow());

        // Start with a viewshed
        player.plot_visibility(&map);
        map.set_visibility(&player.visible_tiles);

        return State{ map: map, player: player, mobs: mobs };
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.position.x + delta_x;
        let new_y = self.player.position.y + delta_y;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.map.is_walkable(new_x, new_y) {

            // Lets see if we are bumping a mob
            for mob in self.mobs.iter() {
                if mob.position.x == new_x && mob.position.y == new_y {
                    // We are
                    return;
                }
            }

            self.player.position.x = new_x;
            self.player.position.y = new_y;
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.map.draw(console);
        self.player.draw(console, &self.map);
        for mob in self.mobs.iter() {
            mob.draw(console, &self.map);
        }

        let mut turn_ended = false;

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }

                328 => { self.move_player(0, -1); turn_ended = true; }
                336 => { self.move_player(0, 1); turn_ended = true; }
                331 => { self.move_player(-1, 0); turn_ended = true; }
                333 => { self.move_player(1, 0); turn_ended = true; }

                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }                
                }
            }
            None => {}
        }

        if turn_ended {
            self.player.plot_visibility(&self.map);
            self.map.set_visibility(&self.player.visible_tiles);
            for mob in self.mobs.iter_mut() {
                mob.plot_visibility(&self.map);
            }
        }
    }
}