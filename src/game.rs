use crate::rltk;
use rltk::Color;
use rltk::Console;

pub enum TileType {
    Wall, Floor
}

pub struct Player {
    pub x : i32,
    pub y : i32
}

pub struct State {
    pub map_tiles : Vec<TileType>,
    pub player : Player
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

        return State{ map_tiles: blank_map, player: Player{ x: 40, y:25 } };
    }

    fn draw_map(&mut self, console : &mut Console) {
        console.cls();

        let mut idx = 0;
        for y in 0 .. 50 {
            for x in 0 .. 80 {
                match self.map_tiles[idx] {
                    TileType::Floor => { console.print_color(x, y, Color::dark_green(), Color::black(), ".".to_string()) }
                    TileType::Wall => { console.print_color(x, y, Color::white(), Color::black(), "#".to_string()) }
                }

                idx += 1;
            }
        }
    }

    fn draw_player(&mut self, console : &mut Console) {
        console.print_color(self.player.x as u32, self.player.y as u32, Color::yellow(), Color::black(), "@".to_string());
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.x + delta_x;
        let new_y = self.player.y + delta_y;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 {
            self.player.x = new_x;
            self.player.y = new_y;
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.draw_map(console);
        self.draw_player(console);

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }

                328 => { self.move_player(0, -1) }
                336 => { self.move_player(0, 1) }
                331 => { self.move_player(-1, 0) }
                333 => { self.move_player(1, 0) }

                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }                
                }
            }
            None => {}
        }
    }
}