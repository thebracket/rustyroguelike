extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum TileType {
    Wall, Floor, Stairs
}
