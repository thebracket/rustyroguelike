use crate::rltk;

pub struct State {
    pub test : i32
}

impl State {
    pub fn new() -> State {
        return State{ test: 0 };
    }

    pub fn tick(&mut self, console : &mut rltk::Console) {
        let timer = format!("{} FPS                 ", console.fps);

        console.print(0, 5, timer);
        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }
                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }
                }
            }
            None => {}
        }

        // Wrapper test
        self.test += 1;
    }
}