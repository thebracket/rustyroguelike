extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Wall, Floor, Stairs
}
