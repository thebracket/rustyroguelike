mod rltk;
mod game;

fn main() {
    let mut gs = game::State::new();
    let mut rltk = rltk::init_with_simple_console(80, 50, "Rusty Roguelike");
    rltk.main_loop(&mut gs);
}
