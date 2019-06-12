mod rltk;

fn callback(console : &mut rltk::Console) {
    let timer = format!("{} FPS                 ", console.fps);

    console.print(0, 5, timer);
    if console.key_pressed {
        match console.key {
            1 => { console.quit() }
            _ =>  { console.print(0,6, format!("You pressed: {}", console.key)) }
        }        
    }
}

fn main() {
    let mut console = rltk::Rltk::init_simple_console(80, 50, "Hello World".to_string());
    console.cls();
    console.main_loop(callback);
}
