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

    fn display_mouse_info(&mut self, console : &mut Console) {
        if self.map.is_tile_visible(&console.mouse_pos) {
            let tile_info = self.map.tile_description(&console.mouse_pos);
            console.print_color(0, 0, Color::cyan(), Color::black(), format!("Tile: {}", tile_info));

            for mob in self.mobs.iter() {
                if mob.position.x == console.mouse_pos.x && mob.position.y == console.mouse_pos.y {
                    console.print_color(0, 1, Color::white(), Color::red(), "Enemy:".to_string());
                    console.print_color(7, 1, Color::red(), Color::black(), format!("{}", mob.name));
                }
            }

            if self.player.position.x == console.mouse_pos.x && self.player.position.y == console.mouse_pos.y {
                console.print_color(0, 1, Color::green(), Color::black(), "It's you!".to_string());
            }
        }

        if console.left_click {
            console.print(0,3, "Clicking won't help you until I support it.".to_string());
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.map.draw(console);
        self.player.draw(console, &self.map);
        for mob in self.mobs.iter() {
            mob.draw(console, &self.map);
        }
        console.set_bg(console.mouse_pos.x as u32, console.mouse_pos.y as u32, Color::magenta());

        self.display_mouse_info(console);

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