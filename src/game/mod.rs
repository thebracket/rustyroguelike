use crate::rltk;
use rltk::Color;
use rltk::Console;
use rltk::Point;

mod tiletype;
pub use tiletype::TileType;

mod ticktype;
pub use ticktype::TickType;

mod fighter;
pub use fighter::Fighter;
pub use fighter::Combat;
pub use fighter::attack;

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

use map::MAX_MOBS_PER_ROOM;
use rand::Rng;

pub struct State {
    pub map : Map,
    pub player : Player,
    pub mobs : Vec<Mob>,
    pub game_state : TickType
}

impl State {
    pub fn new() -> State {
        let mut rng = rand::thread_rng();
        let mut map = Map::new();
        let rooms = map.random_rooms_tut3();

        let (player_x, player_y) = rooms[0].center();

        let mut mobs : Vec<Mob> = Vec::new();
        for i in 1 .. rooms.len() {
            let number_of_mobs = rng.gen_range(1, MAX_MOBS_PER_ROOM+1);
            if number_of_mobs > 0 {
                for _mobn in 1 .. number_of_mobs {
                    let mob_x = rng.gen_range(rooms[i].x1+1, rooms[i].x2-1);
                    let mob_y = rng.gen_range(rooms[i].y1+1, rooms[i].y2-1);

                    let mut found = false;
                    for existing_mob in mobs.iter() {
                        if existing_mob.position.x == mob_x && existing_mob.position.y == mob_y {
                            found = true;
                        }
                    }

                    if !found {
                        let mob = Mob::new_random(mob_x, mob_y, rng.gen_range(1,4));
                        mobs.push(mob);
                    }
                }
            }
        }

        let mut player = Player::new(player_x, player_y, 64, Color::yellow());

        // Start with a viewshed
        player.plot_visibility(&map);
        map.set_visibility(&player.visible_tiles);

        return State{ map: map, player: player, mobs: mobs, game_state: TickType::PlayersTurn };
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.position.x + delta_x;
        let new_y = self.player.position.y + delta_y;
        let mut can_move : bool = true;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.map.is_walkable(new_x, new_y) {

            // Lets see if we are bumping a mob
            for mob in self.mobs.iter_mut() {
                if mob.position.x == new_x && mob.position.y == new_y {
                    // We are
                    let result = attack(&mut self.player, mob);
                    for s in result.iter() {
                        println!("{}", s);
                    }
                    can_move = false;
                }
            }
            self.mobs.retain(|mob| !mob.fighter.dead);

            if can_move {
                self.player.position.x = new_x;
                self.player.position.y = new_y;
            }
        }
    }

    fn display_mouse_info(&mut self, console : &mut Console) {
        if self.map.is_tile_visible(&console.mouse_pos) {
            let tile_info = self.map.tile_description(&console.mouse_pos);
            console.print_color(Point::new(0,0), Color::cyan(), Color::black(), format!("Tile: {}", tile_info));

            for mob in self.mobs.iter() {
                if mob.position == console.mouse_pos {
                    console.print_color(Point::new(0,1), Color::white(), Color::red(), "Enemy:".to_string());
                    console.print_color(Point::new(7,1), Color::red(), Color::black(), format!("{}", mob.name));
                }
            }

            if self.player.position == console.mouse_pos {
                console.print_color(Point::new(0,1), Color::green(), Color::black(), "It's you!".to_string());
            }
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.map.draw(console);
        self.player.draw(console, &self.map);
        for mob in self.mobs.iter() {
            mob.draw(console, &self.map);
        }
        console.set_bg(console.mouse_pos, Color::magenta());

        self.display_mouse_info(console);

        match self.game_state {
            TickType::PlayersTurn => { 
                self.player_tick(console);
            }
            TickType::EnemyTurn => {
                self.mob_tick(console);
                self.game_state = TickType::PlayersTurn; 
            }
        }
        
    }

    fn player_tick(&mut self, console : &mut Console) {
        let mut turn_ended = false;

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }

                // Numpad
                72 => { self.move_player(0, -1); turn_ended = true; }
                75 => { self.move_player(-1, 0); turn_ended = true; }
                77 => { self.move_player(1, 0); turn_ended = true; }
                80 => { self.move_player(0, 1); turn_ended = true; }

                71 => { self.move_player(-1, -1); turn_ended = true; }
                73 => { self.move_player(1, -1); turn_ended = true; }
                79 => { self.move_player(-1, 1); turn_ended = true; }
                81 => { self.move_player(1, 1); turn_ended = true; }

                // Cursors
                328 => { self.move_player(0, -1); turn_ended = true; }
                336 => { self.move_player(0, 1); turn_ended = true; }
                331 => { self.move_player(-1, 0); turn_ended = true; }
                333 => { self.move_player(1, 0); turn_ended = true; }

                _ =>  { println!("You pressed: {}", key) }                
                }
            }
            None => {}
        }

        if turn_ended {
            self.update_visibility();
            self.game_state = TickType::EnemyTurn; 
        }
    }

    fn mob_tick(&mut self, _console: &mut Console) {
        let mut blocked : Vec<bool> = Vec::new();
        for y in 0..50 {
            for x in 0..80 {
                blocked.push(!self.map.is_walkable(x,y));
            }
        }
        blocked[((self.player.position.y * 80) + self.player.position.x) as usize] = true;
        for mob in self.mobs.iter() {
            let idx = ((mob.position.y * 80) + mob.position.x) as usize;
            blocked[idx] = true;
        }

        for mob in self.mobs.iter_mut() {
            mob.turn_tick(&mut self.player, &mut blocked);
        }
        self.update_visibility();
    }

    fn update_visibility(&mut self) {
        self.player.plot_visibility(&self.map);
            self.map.set_visibility(&self.player.visible_tiles);
            for mob in self.mobs.iter_mut() {
                mob.plot_visibility(&self.map);
            }
    }
}