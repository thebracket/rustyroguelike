extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TickType {
    None, PlayersTurn, EnemyTurn, GameOver, UseMenu, DropMenu, TargetingItem
}