mod rltk;

fn callback() {

}

fn main() {
    let mut console = rltk::Rltk::init_simple_console(80, 50, "Hello World".to_string());
    console.cls();
    console.print(0,0, "Hello World".to_string());
    console.print(0,1, "This is an 8x8 console terminal with no interactivity.".to_string());
    console.main_loop(callback);
}
