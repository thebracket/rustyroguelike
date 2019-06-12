mod rltk;

fn main() {
    let (mut ctx, mut console) = rltk::Rltk::init_simple_console(80, 50, "Hello World".to_string());
    console.main_loop(&mut ctx);
}
