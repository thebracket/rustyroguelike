use super::{gui, TickType, inventory, Map, Player, map_builder, Combat, BaseEntity, GameState, rltk, player, mob};
use rltk::{Rltk, Color, Point};

pub struct State {
    pub map : Map,
    pub game_state : TickType,
    pub log : Vec<String>,
    pub entities : Vec<Box<BaseEntity>>,
    pub target_cell : Point,
    pub targeting_item : i32,
    pub prev_mouse_for_targeting : Point,
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        gui::render(self, ctx, &self.map);

        match self.game_state {
            TickType::PlayersTurn => { 
                player::player_tick(self, ctx);
            }
            TickType::EnemyTurn => {
                mob::mob_tick(self, ctx.con());
                self.game_state = TickType::PlayersTurn;
                if self.player().fighter.dead { self.game_state = TickType::GameOver; }
            }
            TickType::GameOver => { gui::display_game_over_and_handle_quit(ctx); }
            TickType::UseMenu => { inventory::use_item(self, ctx); }
            TickType::DropMenu => { inventory::drop_item(self, ctx); }
            TickType::TargetingItem => { inventory::item_targeting(self, ctx); }
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
            entities : entities,
            target_cell : Point::new(-1,-1),
            targeting_item : -1,
            prev_mouse_for_targeting : Point::new(-1,-1)
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
}