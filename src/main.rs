mod rltk;

fn callback(console : &mut rltk::Console) {
    let timer = format!("{} FPS", console.fps);

    console.cls();
    console.print(0, 5, timer);
}

fn main() {
    let mut console = rltk::Rltk::init_simple_console(80, 50, "Hello World".to_string());
    console.main_loop(callback);
}
