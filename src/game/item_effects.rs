use super::{State, BaseEntity};
use crate::rltk;

pub fn use_health_potion(item_index : i32, gs : &mut State, result : &mut Vec<String>) {
    let player = &mut gs.player_mut();
    if player.fighter.hp == player.fighter.max_hp {
        result.push("You are already at maximum health.".to_string());
    } else {
        player.fighter.hp = player.fighter.max_hp; // Cheezed due to confusion over borrowing
        result.push("You are healed!".to_string());
        player.inventory.remove_item_return_clone(item_index);
    }
}

pub fn use_zap_scroll(item_index : i32, gs : &mut State, result : &mut Vec<String>) {
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
    } else {
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