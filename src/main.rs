extern crate rltk;
use rltk::Rltk;
mod game;

fn main() {
    let gs = game::State::new_menu();
    let mut context = Rltk::init_simple8x8(80, 50, "Rusty Roguelike", "resources");
    context.with_post_scanlines(true);
    rltk::main_loop(context, Box::new(gs));
}
