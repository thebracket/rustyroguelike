#[derive(Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x:i32, y:i32) -> Point {
        return Point{x, y};
    }
}
