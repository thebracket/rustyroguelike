use super::{gui, TickType, inventory, Map, Player, map_builder, Combat, BaseEntity, GameState, rltk, player, mob, TileType, Particle, vfx};
use rltk::{Rltk, RGB, Point};
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub map : Map,
    pub game_state : TickType,
    pub log : Vec<String>,
    pub entities : Vec<Box<BaseEntity>>,
    pub target_cell : Point,
    pub targeting_item : i32,
    pub prev_mouse_for_targeting : Point,
    pub menu_state : gui::MenuState,
    pub vfx : Vec<Particle>
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        vfx::age_particles(self, ctx);
        if self.game_state != TickType::MainMenu { gui::render(self, ctx, &self.map); }

        match self.game_state {
            TickType::MainMenu => { 
                let result = gui::display_main_menu(ctx, &mut self.menu_state); 
                match result {
                    gui::MainMenuResult::Quit => { ctx.quit() }
                    gui::MainMenuResult::Continue => {
                        let saved = State::load_saved();
                        self.map = saved.map;
                        self.game_state = saved.game_state;
                        self.log = saved.log;
                        self.entities = saved.entities;
                        self.target_cell = saved.target_cell;
                        self.targeting_item = saved.targeting_item;
                        self.prev_mouse_for_targeting = saved.prev_mouse_for_targeting;
                    }
                    gui::MainMenuResult::New => {
                        let saved = State::new(0);
                        self.map = saved.map;
                        self.game_state = saved.game_state;
                        self.log = saved.log;
                        self.entities = saved.entities;
                        self.target_cell = saved.target_cell;
                        self.targeting_item = saved.targeting_item;
                        self.prev_mouse_for_targeting = saved.prev_mouse_for_targeting;
                    }
                    _ => {}
                }
            }
            TickType::PlayersTurn => { 
                let result = player::player_tick(self, ctx);
                if result == player::PlayerTickResult::NextMap {
                    // Move to next level
                    self.player_mut().dungeon_level += 1;
                    let mut saved = State::new(self.player().dungeon_level);
                    saved.player_mut().copy_from_other_player(self.player());

                    self.map = saved.map;
                    self.entities = saved.entities;
                    self.add_log_entry("You descend to the next level, and take a moment to rest.".to_string());
                }
            }
            TickType::EnemyTurn => {
                mob::mob_tick(self);
                self.game_state = TickType::PlayersTurn;
                if self.player().fighter.dead { 
                    self.game_state = TickType::GameOver; 
                    if Path::new("./savegame.json").exists() { std::fs::remove_file("./savegame.json").expect("Unable to delete file"); } 
                }
            }
            TickType::GameOver => { gui::display_game_over_and_handle_quit(ctx, self); }
            TickType::UseMenu => { inventory::use_item(self, ctx); }
            TickType::DropMenu => { inventory::drop_item(self, ctx); }
            TickType::WieldMenu => { inventory::wield_item(self, ctx); }
            TickType::UnequipMenu => { inventory::unequip_item(self, ctx); }
            TickType::TargetingItem => { inventory::item_targeting(self, ctx); }
            TickType::LevelUpMenu => { gui::handle_level_up(ctx, self); }
            TickType::CharacterMenu => { gui::display_character_info(ctx, self); }
            TickType::HelpMenu => { gui::display_help_info(ctx, self); }
            TickType::None => {}
        }
    }
}

impl State {
    pub fn new_menu() -> State {        
        return State{ 
            map: Map::new(80, 43), 
            game_state: TickType::MainMenu, 
            log: Vec::new(), 
            entities : Vec::new(),
            target_cell : Point::new(-1,-1),
            targeting_item : -1,
            prev_mouse_for_targeting : Point::new(-1,-1),
            menu_state: gui::MenuState::new(),
            vfx : Vec::new()
        };
    }

    pub fn load_saved() -> State {
        let data = fs::read_to_string("./savegame.json").expect("Unable to read file");
        let loaded : State = serde_json::from_str(&data).unwrap();
        std::fs::remove_file("./savegame.json").expect("Unable to delete file");
        return loaded;
    }

    pub fn new(depth: i32) -> State {
        if Path::new("./savegame.json").exists() { std::fs::remove_file("./savegame.json").expect("Unable to delete file"); }

        let mut entities : Vec<Box<BaseEntity>> = Vec::new();
        let mut map = Map::new(80, 43);
        let rooms = map_builder::random_rooms_tut3(&mut map);
        let (player_x, player_y) = rooms[0].center();
        let mobs = map_builder::spawn_mobs(&rooms, depth);
        let items = map_builder::spawn_items(&rooms, &mobs, depth);
        let mut player = Player::new(player_x, player_y, 64, RGB::named(rltk::YELLOW));
        let stairs_pos = rooms[rooms.len()-1].center();
        map.tiles[((stairs_pos.1 * 80) + stairs_pos.0) as usize] = TileType::Stairs;

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
            entities : entities,
            target_cell : Point::new(-1,-1),
            targeting_item : -1,
            prev_mouse_for_targeting : Point::new(-1,-1),
            menu_state : gui::MenuState::new(),
            vfx : Vec::new()
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

    pub fn update_visibility(&mut self) {
        for e in self.entities.iter_mut() {
            e.plot_visibility(&self.map);
        }

        let vt = self.player().visible_tiles.clone();
        self.map.set_visibility(&vt);
    }

    pub fn add_log_entry(&mut self, line : String) {
        self.log.insert(0, line.clone());
        while self.log.len() > 5 { self.log.remove(4); }
    }

    pub fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        let mut f = File::create("./savegame.json").expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }
}