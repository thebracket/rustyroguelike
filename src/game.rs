use crate::rltk;

pub enum TileType {
    Wall, Floor
}

pub struct State {
    pub map_tiles : Vec<TileType>
}

impl State {
    pub fn new() -> State {
        let mut blank_map = Vec::new();
        for _i in 0 .. (80*50) {
            blank_map.push(TileType::Floor);
        }

        for x in 0..80 {
            blank_map[x] = TileType::Wall;
            blank_map[(49*80)+x] = TileType::Wall;
            if x < 49 {
                blank_map[(x*80)] = TileType::Wall;
                blank_map[(x*80)+79] = TileType::Wall;
            }
        }

        return State{ map_tiles: blank_map };
    }

    fn draw_map(&mut self, console : &mut rltk::Console) {
        console.cls();

        let mut idx = 0;
        for y in 0 .. 50 {
            for x in 0 .. 80 {
                match self.map_tiles[idx] {
                    TileType::Floor => { console.print_color(x, y, rltk::Color::dark_green(), rltk::Color::black(), ".".to_string()) }
                    TileType::Wall => { console.print_color(x, y, rltk::Color::white(), rltk::Color::black(), "#".to_string()) }
                }

                idx += 1;
            }
        }
    }

    pub fn tick(&mut self, console : &mut rltk::Console) {
        self.draw_map(console);

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }
                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }
                }
            }
            None => {}
        }
    }
}