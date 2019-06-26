use super::Item;
use super::gui;
use super::gui::ItemMenuResult;
use crate::rltk;
use rltk::Rltk;
use super::State;
use super::TickType;
use super::BaseEntity;

pub struct Inventory {
    pub items : Vec<Item>,
    capacity: i32
}

impl Inventory {
    pub fn new(capacity : i32) -> Inventory {
        return Inventory{ items: Vec::new(), capacity: capacity };
    }

    pub fn add_item(&mut self, item : Item) -> Vec<String> {
        let mut result : Vec<String> = Vec::new();
        if self.items.len() as i32 > self.capacity {
            result.push("You cannot carry any more!".to_string());
        } else {
            result.push(format!("You pick up the {}", item.name));
            self.items.push(item);
        }
        return result;
    }
}

pub fn use_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Use which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let result = gs.player_mut().use_item(selection);
            for s in result.iter() {
                gs.add_log_entry(s.to_string());
            }
            gs.game_state = TickType::PlayersTurn;
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

pub fn drop_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Drop which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let mut item_copy = gs.player_mut().remove_item_from_inventory(selection);
            item_copy.position = gs.player().get_position();
            gs.add_log_entry(format!("You drop the {}", item_copy.name));
            gs.entities.push(Box::new(item_copy));
            gs.game_state = TickType::EnemyTurn;
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}