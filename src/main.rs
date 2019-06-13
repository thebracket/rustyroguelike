mod rltk;
mod game;

fn main() {
    let mut gs = game::State::new();

    let mut console = rltk::Rltk::init_simple_console(80, 50, "Rusty Roguelike".to_string());

    let mut tick_func = |console : &mut rltk::Console| {
        gs.tick(console);
    };

    console.main_loop(&mut tick_func);
}
