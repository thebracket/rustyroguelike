use super::{Item, gui, gui::ItemMenuResult, State, TickType, BaseEntity, player};
use crate::rltk;
use rltk::Rltk;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub items : Vec<Item>,
    pub equipped : Vec<Item>,
    capacity: i32
}

impl Inventory {
    pub fn new(capacity : i32) -> Inventory {
        Inventory{ items: Vec::new(), capacity, equipped: Vec::new() }
    }

    pub fn add_item(&mut self, item : Item) -> Vec<String> {
        let mut result : Vec<String> = Vec::new();
        if self.items.len() as i32 > self.capacity {
            result.push("You cannot carry any more!".to_string());
        } else {
            result.push(format!("You pick up the {}", item.name));
            self.items.push(item);
        }
        result
    }

    pub fn remove_item_return_clone(&mut self, item_index: i32) -> Item {
        let item_copy = self.items[item_index as usize].clone();
        self.items.remove(item_index as usize);
        item_copy
    }

    pub fn get_equippable_items(&self) -> Vec<i32> {
        let mut result = Vec::new();
        for (i,item) in self.items.iter().enumerate() {
            match item.equippable {
                None => {}
                Some(_) => { result.push(i as i32); }
            }
        }
        result
    }
}

pub fn pickup(gs : &mut State) {
    let mut item_index = 0;
    let ppos = gs.player().position;
    for (i,e) in gs.entities.iter_mut().enumerate() {
        if e.can_pickup() && e.get_position() == ppos {
            // We can do it!
            item_index = i;
        }
    }

    if item_index > 0 {
        let cloned_item = gs.entities[item_index].as_item().unwrap().clone();
        let results = gs.player_mut().inventory.add_item(cloned_item); 
        gs.entities.remove(item_index);
        for s in results.iter() {
            gs.add_log_entry(s.clone());
        }
    } else {
        gs.add_log_entry("There is nothing to pick up.".to_string());
    }
}

pub fn use_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Use which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let result = player::use_item(selection, gs);
            for s in result.iter() {
                gs.add_log_entry(s.to_string());
            }            
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

pub fn drop_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Drop which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let mut item_copy = gs.player_mut().inventory.remove_item_return_clone(selection);
            item_copy.position = gs.player().get_position();
            gs.add_log_entry(format!("You drop the {}", item_copy.name));
            gs.entities.push(Box::new(item_copy));
            gs.game_state = TickType::EnemyTurn;
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

pub fn wield_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_equippable_menu(gs, ctx, "Wield which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let result = wield_item_final(selection, gs);
            for s in result.iter() {
                gs.add_log_entry(s.to_string());
            }            
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

fn wield_item_final(item_index : i32, gs : &mut State) -> Vec<String> {
    let mut result = Vec::new();

    let slot = gs.player().inventory.items[item_index as usize].equippable.unwrap().slot;

    // Do we already have anything in that slot? If so, move it to the inventory
    let mut already_equipped : Vec<Item> = Vec::new();
    for equipped in gs.player().inventory.equipped.iter() {
        if equipped.equippable.unwrap().slot == slot {
            result.push(format!("You unequip the {}", equipped.name));
            already_equipped.push(equipped.clone());
        }
    }
    gs.player_mut().inventory.equipped.retain(|a| a.equippable.unwrap().slot != slot);
    for item in already_equipped {
        gs.player_mut().inventory.items.push(item);
    }

    // Put the item in the equip list and remove it from the backpack
    let item = gs.player_mut().inventory.items[item_index as usize].clone();
    result.push(format!("You equip the {}", item.name));
    gs.player_mut().inventory.items.remove(item_index as usize);
    gs.player_mut().inventory.equipped.push(item);
    gs.game_state = TickType::EnemyTurn;

    result
}

pub fn unequip_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_equipped_menu(gs, ctx, "Unequip which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let result = unequip_item_final(selection, gs);
            for s in result.iter() {
                gs.add_log_entry(s.to_string());
            }             
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

fn unequip_item_final(item_index : i32, gs : &mut State) -> Vec<String> {
    let mut result = Vec::new();

    let item = gs.player().inventory.equipped[item_index as usize].clone();
    result.push(format!("You remove the {}", item.name));
    gs.player_mut().inventory.equipped.remove(item_index as usize);
    gs.player_mut().inventory.items.push(item);
    gs.game_state = TickType::EnemyTurn;

    result
}

pub fn item_targeting(gs : &mut State, ctx : &mut Rltk) {
    let result = gui::handle_item_targeting(gs, ctx, "Select your target with cursor keys or mouse, Escape to cancel.");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
        ItemMenuResult::Selected => { player::use_area_item(gs); }
    }
}