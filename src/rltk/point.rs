extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x:i32, y:i32) -> Point {
        return Point{x, y};
    }
}
