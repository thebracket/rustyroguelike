use std::cmp::{min};
use super::Player;
use super::Mob;

pub struct Fighter {
    max_hp : i32,
    hp: i32,
    defense: i32,
    power: i32
}

impl Fighter {
    pub fn new(max_hp: i32, defense: i32, power: i32) -> Fighter {
        return Fighter{
            max_hp: max_hp,
            hp: max_hp,
            defense: defense,
            power: power
        };
    }    
}

pub trait Combat {
    fn attack(&mut self, target: &mut Combat) -> Vec<String>;
    fn get_power(&self)->i32;
    fn get_defense(&self)->i32;
    fn take_damage(&mut self, amount:i32);
    fn heal_damage(&mut self, amount:i32);
    fn get_name(&self)->String;
    fn get_hp(&self)->i32;
}

impl Combat for Player {
    fn attack(&mut self, target: &mut Combat) -> Vec<String> {
        let mut results = Vec::new();

        let damage = self.get_power() - target.get_defense();
        if damage > 0 {
            target.take_damage(damage);
            results.push(format!("{} attacks {}, for {} hit points of damage.", self.get_name(), target.get_name(), damage));
            results.push(format!("{} has {} remaining hit points.", target.get_name(), target.get_hp()));
            if (target.get_hp() < 1) {
                results.push(format!("{} would be dead if we supported that.", target.get_name()));
            }
        } else {
            results.push(format!("{} attacks {}, but lacks the power to do anything useful.", self.get_name(), target.get_name()));
        }

        return results;
    }

    fn take_damage(&mut self, amount:i32) {
        self.fighter.hp -= amount;
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

}

impl Combat for Mob {
    fn attack(&mut self, target: &mut Combat) -> Vec<String> {
        let mut results = Vec::new();

        let damage = self.get_power() - target.get_defense();
        if damage > 0 {
            target.take_damage(damage);
            results.push(format!("{} attacks {}, for {} hit points of damage.", self.get_name(), target.get_name(), damage));
            if (target.get_hp() < 1) {
                results.push(format!("{} would be dead if we supported that.", target.get_name()));
            }
        } else {
            results.push(format!("{} attacks {}, but lacks the power to do anything useful.", self.get_name(), target.get_name()));
        }

        return results;
    }

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
}