use super::gui;
use super::TickType;
use super::inventory;
use super::Map;
use super::Player;
use super::attack;
use super::map_builder;
use super::Combat;
use super::Console;
use super::BaseEntity;
use super::GameState;
use crate::rltk;
use super::player;
use rltk::Rltk;
use rltk::Color;
use rltk::Algorithm2D;

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
                player::player_tick(self, ctx);
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
        let rooms = map_builder::random_rooms_tut3(&mut map);
        let (player_x, player_y) = rooms[0].center();
        let mobs = map_builder::spawn_mobs(&rooms);
        let items = map_builder::spawn_items(&rooms, &mobs);       
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
}