use super::Console;

pub trait GameState {
    fn tick(&mut self, console : &mut Console);
}
