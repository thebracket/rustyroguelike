use crate::rltk;
use rltk::{Color, Point, Rltk, field_of_view};
use super::{fighter::Fighter, Inventory, BaseEntity, Combat, Map, ItemType, State, attack, TickType, inventory};

pub struct Player {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub visible_tiles : Vec<Point>,
    pub fighter : Fighter,
    pub inventory : Inventory
}

impl Player {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Player {
        Player{ 
            position: Point::new(x, y), 
            glyph, fg, 
            visible_tiles: Vec::new(), 
            fighter: Fighter::new(8, 0, 1),
            inventory: Inventory::new(4)
        }
    }    
}

impl BaseEntity for Player {
    fn get_position(&self) -> Point { self.position }
    fn get_fg_color(&self) -> Color { self.fg }
    fn get_glyph(&self) -> u8 { self.glyph }
    fn as_player(&self) -> Option<&Player> { Some(self) }
    fn as_player_mut(&mut self) -> Option<&mut Player> { Some(self) }
    fn as_combat(&mut self) -> Option<&mut Combat> { Some(self) }
    fn plot_visibility(&mut self, map : &Map) {
        self.visible_tiles = field_of_view(self.get_position(), 6, map);
    }
    fn get_tooltip_text(&self) -> String { "It's you!".to_string() }
    fn get_name(&self) -> String { "Player".to_string() }
}

// Handlers for gameplay

pub fn player_tick(gs : &mut State, ctx : &mut Rltk) {
    let mut turn_ended = false;
    let mut attack_target : Option<usize> = None;

    match ctx.key {
        Some(key) => {
            match key {
            glfw::Key::Escape => { ctx.quit() }

            // Numpad
            glfw::Key::Kp8 => { attack_target = move_player(gs, 0, -1); turn_ended = true; }
            glfw::Key::Kp4 => { attack_target = move_player(gs, -1, 0); turn_ended = true; }
            glfw::Key::Kp6 => { attack_target = move_player(gs, 1, 0); turn_ended = true; }
            glfw::Key::Kp2 => { attack_target = move_player(gs, 0, 1); turn_ended = true; }

            glfw::Key::Kp7 => { attack_target = move_player(gs, -1, -1); turn_ended = true; }
            glfw::Key::Kp9 => { attack_target = move_player(gs, 1, -1); turn_ended = true; }
            glfw::Key::Kp1 => { attack_target = move_player(gs, -1, 1); turn_ended = true; }
            glfw::Key::Kp3 => { attack_target = move_player(gs, 1, 1); turn_ended = true; }

            // Cursors
            glfw::Key::Up => { attack_target = move_player(gs, 0, -1); turn_ended = true; }
            glfw::Key::Down => { attack_target = move_player(gs, 0, 1); turn_ended = true; }
            glfw::Key::Left => { attack_target = move_player(gs, -1, 0); turn_ended = true; }
            glfw::Key::Right => { attack_target = move_player(gs, 1, 0); turn_ended = true; }

            // Wait
            glfw::Key::Kp5 => { turn_ended = true; }

            // Pick up
            glfw::Key::G => { inventory::pickup(gs); turn_ended = true; }

            // Use/drop items
            glfw::Key::U => { use_menu(gs); }
            glfw::Key::D => { drop_menu(gs); }

            _ =>  { }
            }
        }
        None => {}
    }

    match attack_target {
        Some(target) => { 
            let player = gs.player_as_combat();
            let result = attack(player.get_name(), player.get_power(), gs.entities[target].as_combat().unwrap());
            for s in result {
                gs.add_log_entry(s.to_string());
            }
            gs.entities.retain(|e| !e.is_dead());
            }
        _ => {}
    }

    if turn_ended {
        gs.update_visibility();
        gs.game_state = TickType::EnemyTurn; 
    }
}

// Returns the ID of the target if we're attacking
fn move_player(gs : &mut State, delta_x : i32, delta_y: i32) -> Option<usize> {
    let mut result : Option<usize> = None;
    let new_x = gs.player().position.x + delta_x;
    let new_y = gs.player().position.y + delta_y;
    let mut can_move : bool = true;
    if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && gs.map.is_walkable(new_x, new_y) {

        // Lets see if we are bumping a mob
        let new_pos = Point::new(new_x, new_y);
        let mut i : usize = 0;
        for e in gs.entities.iter_mut() {
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
            gs.player_mut().position.x = new_x;
            gs.player_mut().position.y = new_y;
        }
    }
    return result;
}

fn use_menu(gs : &mut State) {
    if gs.player().inventory.items.is_empty() {
        gs.add_log_entry("You don't have any usable items".to_string());
    } else {
        gs.game_state = TickType::UseMenu;
    }
}

fn drop_menu(gs : &mut State) {
    if gs.player().inventory.items.is_empty() {
        gs.add_log_entry("You don't have any items to drop!".to_string());
    } else {
        gs.game_state = TickType::DropMenu;
    }
}    

pub fn use_item(item_index : i32, gs : &mut State) -> Vec<String> {
    let mut result = Vec::new();

    let item_type = gs.player().inventory.items[item_index as usize].item_type;
    match item_type {
        ItemType::HealthPotion => {
            let player = &mut gs.player_mut();
            if player.fighter.hp == player.fighter.max_hp {
                result.push("You are already at maximum health.".to_string());
                return result;
            }

            player.fighter.hp = player.fighter.max_hp; // Cheezed due to confusion over borrowing
            result.push("You are healed!".to_string());
            player.inventory.remove_item_return_clone(item_index);
        }

        ItemType::ZapScroll => {
            let mut possible_targets : Vec<(usize, f32)> = Vec::new();
            let visible_tiles = gs.player().visible_tiles.clone();
            let my_pos = gs.player().get_position();
            let mut i : usize = 0;
            for potential_target in gs.entities.iter() {
                if potential_target.is_mob() {
                    let target_pos = potential_target.get_position();
                    if visible_tiles.contains(&target_pos) {
                        possible_targets.push((i, rltk::distance2d(my_pos, target_pos)));
                    }
                }
                i += 1;
            }

            if possible_targets.is_empty() {
                result.push("You can't see anyone to zap, so you put the scroll away.".to_string());
                return result;
            }

            possible_targets.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            let mut target = &mut gs.entities[possible_targets[0].0].as_mob_mut().unwrap();
            result.push(format!("Lightning from the scroll zaps {} for 8 points of damage.", target.name));
            target.fighter.hp -= 8;
            if target.fighter.hp < 1 { 
                target.fighter.dead = true; 
                result.push(format!("{} is burned to a crisp.", target.name));
            }
            gs.entities.retain(|e| !e.is_dead());

            // Remove the scroll
            gs.player_mut().inventory.remove_item_return_clone(item_index);
        }
    }

    return result;
}