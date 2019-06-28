use std::cmp::{min};
use super::{Player, Mob};
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Fighter {
    pub max_hp : i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub dead: bool,
    pub xp_value : i32
}

impl Fighter {
    pub fn new(max_hp: i32, defense: i32, power: i32, xp:i32) -> Fighter {
        return Fighter{
            max_hp: max_hp,
            hp: max_hp,
            defense: defense,
            power: power,
            dead: false,
            xp_value : xp
        };
    }    
}

pub trait Combat {
    fn get_power(&self)->i32;
    fn get_defense(&self)->i32;
    fn take_damage(&mut self, amount:i32);
    fn heal_damage(&mut self, amount:i32);
    fn get_name(&self)->String;
    fn get_hp(&self)->i32;
    fn kill(&mut self);
    fn xp_value(&self)->i32 { 0 }
}

pub fn attack(instigator_name: String, instigator_power : i32, target: &mut Combat) -> (i32, Vec<String>) {
    let mut results = Vec::new();
    let mut xp = 0;

    let damage = instigator_power - target.get_defense();
    if damage > 0 {
        target.take_damage(damage);
        results.push(format!("{} attacks {}, for {} hit points of damage.", instigator_name, target.get_name(), damage));
        results.push(format!("{} has {} remaining hit points.", target.get_name(), target.get_hp()));
        if target.get_hp() < 1 {
            results.push(format!("{} is dead.", target.get_name()));
            target.kill();
            xp += target.xp_value();
        }
    } else {
        results.push(format!("{} attacks {}, but lacks the power to do anything useful.", instigator_name, target.get_name()));
    }

    return (xp, results);
}

impl Combat for Player {
    fn take_damage(&mut self, amount:i32) {
        self.fighter.hp -= amount;
    }

    fn heal_damage(&mut self, amount:i32) {
        self.fighter.hp = min(self.fighter.max_hp, self.fighter.hp + amount);
    }

    fn get_name(&self) -> String {
        return "Player".to_string();
    }

    fn get_defense(&self) -> i32 { 
        let mut defense = self.fighter.defense;
        for item in self.inventory.equipped.iter() {
            defense += item.equippable.unwrap().defense_bonus;
        }
        return defense; 
    }

    fn get_power(&self) -> i32 { 
        let mut power = self.fighter.power;
        for item in self.inventory.equipped.iter() {
            power += item.equippable.unwrap().power_bonus;
        }
        return power; 
    }

    fn get_hp(&self) -> i32 { return self.fighter.hp; }
    fn kill(&mut self) { self.fighter.dead = true; }
}

impl Combat for Mob {
    fn take_damage(&mut self, amount:i32) {
        self.fighter.hp -= amount;
    }

    fn heal_damage(&mut self, amount:i32) {
        self.fighter.hp = min(self.fighter.max_hp, self.fighter.hp + amount);
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_defense(&self) -> i32 { return self.fighter.defense; }
    fn get_power(&self) -> i32 { return self.fighter.power; }
    fn get_hp(&self) -> i32 { return self.fighter.hp; }
    fn kill(&mut self) { self.fighter.dead = true; }
    fn xp_value(&self)->i32 { self.fighter.xp_value }
}