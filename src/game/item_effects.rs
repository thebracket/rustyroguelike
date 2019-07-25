use super::{State, BaseEntity, TickType, Combat, Particle};
use crate::rltk;
use rltk::{RGB};

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
                possible_targets.push((i, rltk::distance2d(rltk::DistanceAlg::Pythagoras, my_pos, target_pos)));
            }
        }
        i += 1;
    }

    if possible_targets.is_empty() {
        result.push("You can't see anyone to zap, so you put the scroll away.".to_string());
    } else {
        possible_targets.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        let target = &mut gs.entities[possible_targets[0].0].as_mob_mut().unwrap();

        let tp = target.get_position();
        let line = rltk::line2d(rltk::LineAlg::Bresenham, tp, my_pos);
        for zap in line {
            gs.vfx.push(Particle::new(zap, RGB::named(rltk::CYAN), RGB::named(rltk::BLACK), 15, 200.0));
        }

        result.push(format!("Lightning from the scroll zaps {} for 8 points of damage.", target.name));
        target.take_damage(8);
        if target.fighter.hp < 1 { 
            target.kill();
            result.push(format!("{} is burned to a crisp.", target.name));
            gs.player_mut().xp += target.fighter.xp_value;
        }
        gs.entities.retain(|e| !e.is_dead());

        // Remove the scroll
        gs.player_mut().inventory.remove_item_return_clone(item_index);
    }
}

pub fn use_fireball_scroll(gs : &mut State, result : &mut Vec<String>) {
    result.push("You launch a fireball!".to_string());

    let target = gs.target_cell;
    let item_index = gs.targeting_item;

    let area_of_effect = rltk::field_of_view(target, 3, &gs.map);
    for pos in area_of_effect.iter() {
        gs.vfx.push(Particle::new(*pos, RGB::named(rltk::RED), RGB::named(rltk::YELLOW), 176, 200.0));
    }
    let mut targets : Vec<usize> = Vec::new();
    let mut i : usize = 0;
    for e in gs.entities.iter() {
        if area_of_effect.contains(&e.get_position()) && e.can_be_attacked() { targets.push(i); }
        i += 1;
    }

    for target_id in targets {
        let target = gs.entities[target_id].as_combat();
        match target {
            None => {}
            Some(target) => {
                result.push(format!("{} is burned for 8 points of damage.", target.get_name()));
                target.take_damage(8);
                if target.get_hp() < 1 { 
                    result.push(format!("{} is dead.", target.get_name()));
                    target.kill();
                    gs.player_mut().xp += target.xp_value();
                }
            }
        }
    }

    gs.entities.retain(|e| !(e.is_dead() && !e.is_player()));

    // Remove the scroll
    gs.player_mut().inventory.remove_item_return_clone(item_index);
    gs.game_state = TickType::EnemyTurn;

    for r in result {
        gs.add_log_entry(r.to_string());
    }
}

pub fn use_confusion_scroll(item_index : i32, gs : &mut State, result : &mut Vec<String>) {
    let mut possible_targets : Vec<(usize, f32)> = Vec::new();
    let visible_tiles = gs.player().visible_tiles.clone();
    let my_pos = gs.player().get_position();
    let mut i : usize = 0;
    for potential_target in gs.entities.iter() {
        if potential_target.is_mob() {
            let target_pos = potential_target.get_position();
            if visible_tiles.contains(&target_pos) {
                possible_targets.push((i, rltk::distance2d(rltk::DistanceAlg::Pythagoras, my_pos, target_pos)));
            }
        }
        i += 1;
    }

    if possible_targets.is_empty() {
        result.push("You can't see anyone to zap, so you put the scroll away.".to_string());
    } else {
        possible_targets.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        let mut target = &mut gs.entities[possible_targets[0].0].as_mob_mut().unwrap();
        result.push(format!("{} is confused.", target.name));
        target.confused = Some(5);

        // Remove the scroll
        gs.player_mut().inventory.remove_item_return_clone(item_index);
    }
}