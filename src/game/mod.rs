use crate::rltk;
use rltk::Color;
use rltk::Rltk;
use rltk::Console;
use rltk::Point;
use rltk::GameState;

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

mod map_builder;
use map_builder::random_rooms_tut3;
use map_builder::spawn_mobs;

pub struct State {
    pub map : Map,
    pub player : Player,
    pub mobs : Vec<Mob>,
    pub game_state : TickType,
    pub log : Vec<String>
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.map.draw(ctx.con());
        self.player.draw(ctx.con(), &self.map);
        for mob in self.mobs.iter() {
            mob.draw(ctx.con(), &self.map);
        }
        ctx.consoles[0].set_bg(ctx.mouse_pos, Color::magenta());
        self.draw_ui(ctx.con());

        self.display_mouse_info(ctx);

        match self.game_state {
            TickType::PlayersTurn => { 
                self.player_tick(ctx);
            }
            TickType::EnemyTurn => {
                self.mob_tick(ctx.con());
                self.game_state = TickType::PlayersTurn;
                if self.player.fighter.dead { self.game_state = TickType::GameOver; }
            }
            TickType::GameOver => {
                ctx.con().print(Point::new(10, 10),"You are dead.".to_string());
            }
        }
    }
}

impl State {
    pub fn new() -> State {
        let mut map = Map::new(80, 43);
        let rooms = random_rooms_tut3(&mut map);
        let (player_x, player_y) = rooms[0].center();
        let mobs = spawn_mobs(&rooms);
        let mut player = Player::new(player_x, player_y, 64, Color::yellow());

        // Start with a viewshed
        player.plot_visibility(&map);
        map.set_visibility(&player.visible_tiles);

        return State{ map: map, player: player, mobs: mobs, game_state: TickType::PlayersTurn, log: Vec::new() };
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.position.x + delta_x;
        let new_y = self.player.position.y + delta_y;
        let mut can_move : bool = true;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.map.is_walkable(new_x, new_y) {

            // Lets see if we are bumping a mob
            let mut tmp : Vec<String> = Vec::new();
            for mob in self.mobs.iter_mut() {
                if mob.position.x == new_x && mob.position.y == new_y {
                    // We are
                    let result = attack(&mut self.player, mob);
                    for s in result.iter() {
                        let tmp_str : String = (*s.clone()).to_string();
                        tmp.push(tmp_str);
                    }
                    can_move = false;
                }
            }
            self.mobs.retain(|mob| !mob.fighter.dead);

            if can_move {
                self.player.position.x = new_x;
                self.player.position.y = new_y;
            }

            for s in tmp {
                self.add_log_entry(s);
            }
        }
    }

    fn display_mouse_info(&mut self, ctx : &mut Rltk) {
        if self.map.is_tile_visible(&ctx.mouse_pos) {
            let tile_info = self.map.tile_description(&ctx.mouse_pos);
            ctx.con().print_color(Point::new(0,0), Color::cyan(), Color::black(), format!("Tile: {}", tile_info));

            for mob in self.mobs.iter() {
                if mob.position == ctx.mouse_pos {
                    ctx.con().print_color(Point::new(0,1), Color::white(), Color::red(), "Enemy:".to_string());
                    ctx.con().print_color(Point::new(7,1), Color::red(), Color::black(), format!("{}", mob.name));
                }
            }

            if self.player.position == ctx.mouse_pos {
                ctx.con().print_color(Point::new(0,1), Color::green(), Color::black(), "It's you!".to_string());
            }
        }
    }

    fn player_tick(&mut self, ctx : &mut Rltk) {
        let mut turn_ended = false;

        match ctx.key {
            Some(key) => {
                match key {
                //1 => { console.quit() }

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
        self.map.refresh_blocked();
        //self.map.set_tile_blocked((self.player.position.y * 80) + self.player.position.x);
        for mob in self.mobs.iter() { self.map.set_tile_blocked((mob.position.y * 80) + mob.position.x); }

        let mut tmp : Vec<String> = Vec::new();
        for mob in self.mobs.iter_mut() {
            let result = mob.turn_tick(&mut self.player, &mut self.map);
            for s in result {
                tmp.push(s.clone().to_string());
            }
        }
        self.update_visibility();
        for s in tmp {
            self.add_log_entry(s);
        }
    }

    fn update_visibility(&mut self) {
        self.player.plot_visibility(&self.map);
            self.map.set_visibility(&self.player.visible_tiles);
            for mob in self.mobs.iter_mut() {
                mob.plot_visibility(&self.map);
            }
    }

    fn draw_ui(&self, console: &mut Console) {
        console.draw_box(Point::new(1, 43), 78, 6, Color::white(), Color::black());
        let health = format!(" HP: {} / {} ", self.player.fighter.hp, self.player.fighter.max_hp);
        console.print_color(Point::new(3,43), Color::yellow(), Color::black(), health);

        console.draw_bar_horizontal(Point::new(20, 43), 59, self.player.fighter.hp, self.player.fighter.max_hp, Color::red(), Color::black());

        let mut y = 44;
        for s in self.log.iter() {
            console.print(Point::new(2, y), s.to_string());
            y += 1;
        }
    }

    fn add_log_entry(&mut self, line : String) {
        self.log.insert(0, line.clone());
        while self.log.len() > 5 { self.log.remove(4); }
    }
}