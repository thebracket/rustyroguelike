use std::cmp::{min};
use super::Player;
use super::Mob;

pub struct Fighter {
    pub max_hp : i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub dead: bool
}

impl Fighter {
    pub fn new(max_hp: i32, defense: i32, power: i32) -> Fighter {
        return Fighter{
            max_hp: max_hp,
            hp: max_hp,
            defense: defense,
            power: power,
            dead: false
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
}

pub fn attack(instigator: &mut Combat, target: &mut Combat) -> Vec<String> {
    let mut results = Vec::new();

    let damage = instigator.get_power() - target.get_defense();
    if damage > 0 {
        target.take_damage(damage);
        results.push(format!("{} attacks {}, for {} hit points of damage.", instigator.get_name(), target.get_name(), damage));
        results.push(format!("{} has {} remaining hit points.", target.get_name(), target.get_hp()));
        if target.get_hp() < 1 {
            results.push(format!("{} is dead.", target.get_name()));
            target.kill();
        }
    } else {
        results.push(format!("{} attacks {}, but lacks the power to do anything useful.", instigator.get_name(), target.get_name()));
    }

    return results;
}

impl Combat for Player {
    fn take_damage(&mut self, amount:i32) {
        //self.fighter.hp -= amount;
    }

    fn heal_damage(&mut self, amount:i32) {
        self.fighter.hp = min(self.fighter.max_hp, self.fighter.hp + amount);
    }

    fn get_name(&self) -> String {
        return "Player".to_string();
    }

    fn get_defense(&self) -> i32 { return self.fighter.defense; }
    fn get_power(&self) -> i32 { return self.fighter.power; }
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
}