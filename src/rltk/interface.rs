use super::Rltk;
use super::Point;

pub trait GameState {
    fn tick(&mut self, ctx : &mut Rltk);
}

pub trait Algorithm2D {
    fn point2d_to_index(&self, pt : Point) -> i32;
    fn index_to_point2d(&self, idx:i32) -> Point;
    fn can_see_through_tile(&self, idx: i32) -> bool;
    fn get_available_exits(&self, idx:i32) -> Vec<(i32, f32)>;
}
