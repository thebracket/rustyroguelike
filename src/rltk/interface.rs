use super::Console;
use super::Point;

pub trait GameState {
    fn tick(&mut self, console : &mut Console);
}

pub trait TileVisibility {
    fn can_see_through_tile(&self, idx: i32) -> bool;
    fn point2d_to_index(&self, pt : Point) -> i32;
}