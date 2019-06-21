use super::Console;

pub trait GameState {
    fn tick(&mut self, console : &mut Console);
}

pub trait TileBlocked {
    fn is_blocked(&mut self, idx: i32);
}