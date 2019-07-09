extern crate rltk;
use rltk::Rltk;
mod game;

fn main() {
    let gs = game::State::new_menu();
    let context = Rltk::init_simple8x8(80, 50, "Hello RLTK World", "resources");
    rltk::main_loop(context, Box::new(gs));
}
