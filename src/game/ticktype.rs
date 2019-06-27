extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum TickType {
    None, MainMenu, PlayersTurn, EnemyTurn, GameOver, UseMenu, DropMenu, TargetingItem, LevelUpMenu
}