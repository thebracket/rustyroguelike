use crate::rltk;
use rltk::Color;
use rltk::Rltk;
use rltk::Console;
use rltk::Point;
use rltk::GameState;
use rltk::Algorithm2D;

mod entity;
pub use entity::BaseEntity;

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

mod map;
pub use map::Map;

mod item;
use item::Item;
use item::ItemType;

mod inventory;
use inventory::Inventory;

extern crate rand;

mod map_builder;
use map_builder::random_rooms_tut3;
use map_builder::spawn_mobs;
use map_builder::spawn_items;

mod gui;

pub struct State {
    pub map : Map,
    pub game_state : TickType,
    pub log : Vec<String>,
    pub entities : Vec<Box<BaseEntity>>
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        gui::render(self, ctx, &self.map);

        match self.game_state {
            TickType::PlayersTurn => { 
                self.player_tick(ctx);
            }
            TickType::EnemyTurn => {
                self.mob_tick(ctx.con());
                self.game_state = TickType::PlayersTurn;
                if self.player().fighter.dead { self.game_state = TickType::GameOver; }
            }
            TickType::GameOver => {
                gui::display_game_over_and_handle_quit(ctx);
            }
            TickType::UseMenu => {
                inventory::use_item(self, ctx);
            }
            TickType::DropMenu => {
                inventory::drop_item(self, ctx);
            }
            TickType::None => {}
        }
    }
}

impl State {
    pub fn new() -> State {
        let mut entities : Vec<Box<BaseEntity>> = Vec::new();
        let mut map = Map::new(80, 43);
        let rooms = random_rooms_tut3(&mut map);
        let (player_x, player_y) = rooms[0].center();
        let mobs = spawn_mobs(&rooms);
        let items = spawn_items(&rooms, &mobs);       
        let mut player = Player::new(player_x, player_y, 64, Color::yellow());

        // Start with a viewshed
        player.plot_visibility(&map);
        map.set_visibility(&player.visible_tiles);

        entities.push(Box::new(player));
        for m in mobs {
            entities.push(Box::new(m));
        }
        for i in items {
            entities.push(Box::new(i));
        }

        return State{ 
            map: map, 
            game_state: TickType::PlayersTurn, 
            log: Vec::new(), 
            entities : entities
        };
    }

    pub fn player(&self) -> &Player {
        return self.entities[0].as_player().unwrap();
    }

    pub fn player_mut(&mut self) -> &mut Player {
        return self.entities[0].as_player_mut().unwrap();
    }

    pub fn player_as_combat(&mut self) -> &mut Combat {
        return self.entities[0].as_combat().unwrap();
    }

    // Returns the ID of the target if we're attacking
    fn move_player(&mut self, delta_x : i32, delta_y: i32) -> Option<usize> {
        let mut result : Option<usize> = None;
        let new_x = self.player().position.x + delta_x;
        let new_y = self.player().position.y + delta_y;
        let mut can_move : bool = true;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.map.is_walkable(new_x, new_y) {

            // Lets see if we are bumping a mob
            let new_pos = Point::new(new_x, new_y);
            let mut i : usize = 0;
            for e in self.entities.iter_mut() {
                if e.get_position() == new_pos && e.blocks_tile() {
                    // Tile is indeed blocked
                    can_move = false;
                    if e.can_be_attacked() {
                        // Attack it!
                        result = Some(i);
                    }
                }
                i += 1;
            }

            if can_move {
                self.player_mut().position.x = new_x;
                self.player_mut().position.y = new_y;
            }
        }
        return result;
    }

    fn pickup(&mut self) {
        let mut i = 0;
        let mut item_index = 0;
        let ppos = self.player().position;
        for e in self.entities.iter_mut() {
            if e.can_pickup() && e.get_position() == ppos {
                // We can do it!
                item_index = i;
            }
            i += 1;
        }

        if item_index > 0 {
            let cloned_item = self.entities[item_index].as_item().unwrap().clone();
            let results = self.player_mut().inventory.add_item(cloned_item); 
            self.entities.remove(item_index);
            for s in results.iter() {
                self.add_log_entry(s.clone());
            }
        }
    }

    fn use_menu(&mut self) {
        if self.player().inventory.items.is_empty() {
            self.add_log_entry("You don't have any usable items".to_string());
        } else {
            self.game_state = TickType::UseMenu;
        }
    }

    fn drop_menu(&mut self) {
        if self.player().inventory.items.is_empty() {
            self.add_log_entry("You don't have any items to drop!".to_string());
        } else {
            self.game_state = TickType::DropMenu;
        }
    }

    fn player_tick(&mut self, ctx : &mut Rltk) {
        let mut turn_ended = false;
        let mut attack_target : Option<usize> = None;

        match ctx.key {
            Some(key) => {
                match key {
                glfw::Key::Escape => { ctx.quit() }

                // Numpad
                glfw::Key::Kp8 => { attack_target = self.move_player(0, -1); turn_ended = true; }
                glfw::Key::Kp4 => { attack_target = self.move_player(-1, 0); turn_ended = true; }
                glfw::Key::Kp6 => { attack_target = self.move_player(1, 0); turn_ended = true; }
                glfw::Key::Kp2 => { attack_target = self.move_player(0, 1); turn_ended = true; }

                glfw::Key::Kp7 => { attack_target = self.move_player(-1, -1); turn_ended = true; }
                glfw::Key::Kp9 => { attack_target = self.move_player(1, -1); turn_ended = true; }
                glfw::Key::Kp1 => { attack_target = self.move_player(-1, 1); turn_ended = true; }
                glfw::Key::Kp3 => { attack_target = self.move_player(1, 1); turn_ended = true; }

                // Cursors
                glfw::Key::Up => { attack_target = self.move_player(0, -1); turn_ended = true; }
                glfw::Key::Down => { attack_target = self.move_player(0, 1); turn_ended = true; }
                glfw::Key::Left => { attack_target = self.move_player(-1, 0); turn_ended = true; }
                glfw::Key::Right => { attack_target = self.move_player(1, 0); turn_ended = true; }

                // Wait
                glfw::Key::Kp5 => { turn_ended = true; }

                // Pick up
                glfw::Key::G => { self.pickup(); turn_ended = true; }

                // Use/drop items
                glfw::Key::U => { self.use_menu(); }
                glfw::Key::D => { self.drop_menu(); }

                _ =>  { }
                }
            }
            None => {}
        }

        match attack_target {
            Some(target) => { 
                let player = self.player_as_combat();
                let result = attack(player.get_name(), player.get_power(), self.entities[target].as_combat().unwrap());
                for s in result {
                    self.add_log_entry(s.to_string());
                }
                self.entities.retain(|e| !e.is_dead());
             }
            _ => {}
        }

        if turn_ended {
            self.update_visibility();
            self.game_state = TickType::EnemyTurn; 
        }
    }

    fn mob_tick(&mut self, _console: &mut Console) {
        // Build the master map of unavailable tiles
        self.map.refresh_blocked();
        for e in self.entities.iter() {
            if e.blocks_tile() {
                let pos = e.get_position();
                self.map.set_tile_blocked(self.map.point2d_to_index(pos));
            }
        }

        let mut i : usize = 0;
        let mut active_mobs : Vec<usize> = Vec::new();
        for e in self.entities.iter_mut() {
            if e.is_mob() { active_mobs.push(i); }
            i += 1;
        }

        let ppos = self.player().position;
        let mut attacking_mobs : Vec<usize> = Vec::new();

        for id in active_mobs {
            let mob = self.entities[id].as_mob_mut().unwrap();
            if mob.turn_tick(ppos, &mut self.map) {
                attacking_mobs.push(id);
            }
        }

        let mut tmp : Vec<String> = Vec::new();
        for id in attacking_mobs {
            let attacker_name = self.entities[id].get_name();
            let attacker_power = self.entities[id].as_combat().unwrap().get_power();
            let result = attack(attacker_name, attacker_power, self.player_as_combat());
            for r in result {
                tmp.push(r);
            }
        }
        for s in tmp {
            self.add_log_entry(s);
        }
    }

    fn update_visibility(&mut self) {
        for e in self.entities.iter_mut() {
            e.plot_visibility(&self.map);
        }

        //self.player_mut().plot_visibility(&self.map);
        let vt = self.player().visible_tiles.clone();
        self.map.set_visibility(&vt);
        //for mob in self.mobs.iter_mut() {
        //    mob.plot_visibility(&self.map);
        //}
    }

    fn add_log_entry(&mut self, line : String) {
        self.log.insert(0, line.clone());
        while self.log.len() > 5 { self.log.remove(4); }
    }
}