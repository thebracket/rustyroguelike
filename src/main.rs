mod rltk;
mod game;

fn main() {
    let mut gs = game::State::new();
    let mut console = rltk::init_simple_console(80, 50, "Rusty Roguelike");
    console.main_loop(&mut gs);
}
