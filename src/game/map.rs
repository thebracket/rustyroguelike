use crate::rltk;
use rltk::Color;
use rltk::Console;
use rltk::Point;

use super::TileType;

pub struct Map {
    pub tiles : Vec<TileType>,
    pub visible : Vec<bool>,
    pub revealed : Vec<bool>
}

impl Map {
    pub fn new() -> Map {
        let mut visible = Vec::new();
        let mut blank_map = Vec::new();
        let mut revealed = Vec::new();
        for _i in 0 .. (80*50) {
            blank_map.push(TileType::Wall);
            visible.push(false);
            revealed.push(false);
        }

        return Map{tiles : blank_map, visible: visible, revealed: revealed};
    }

    pub fn set_visibility(&mut self, vis : &Vec<Point>) {
        for v in self.visible.iter_mut() {
            *v = false;
        }

        for pt in vis {
            let idx = self.tile_idx(pt.x, pt.y);
            match idx {
                Some(x) => { self.visible[x] = true; self.revealed[x] = true; }
                None => {}
            }
        }
    }    

    pub fn draw(&mut self, console : &mut Console) {
        console.cls();

        let mut idx = 0;
        for y in 0 .. 50 {
            for x in 0 .. 80 {

                // You wouldn't normally make this mess - clean up!
                let coord = Point::new(x, y);
                if self.revealed[idx] {
                    if self.visible[idx] {
                        match self.tiles[idx] {
                            TileType::Floor => { console.print_color(coord, Color::dark_green(), Color::black(), ".".to_string()) }
                            TileType::Wall => { console.print_color(coord, Color::white(), Color::black(), "#".to_string()) }
                        }
                    } else {
                        match self.tiles[idx] {
                            TileType::Floor => { console.print_color(coord, Color::grey(), Color::black(), ".".to_string()) }
                            TileType::Wall => { console.print_color(coord, Color::grey(), Color::black(), "#".to_string()) }
                        }
                    }
                }

                idx += 1;
            }
        }
    }

    // Utility function: find the index of a tile at x/y
    fn tile_idx(&self, x:i32, y:i32) -> Option<usize> {
        if self.valid_tile(x, y) {
            return Some(((y*80)+x) as usize);
        } else {
            return None;
        }
    }

    // Utility function: bounds checking
    fn valid_tile(&self, x:i32, y:i32) -> bool {
        return x > 0 && x < 79 && y > 0 && y < 49;
    }

    // Utility function: is a tile walkable
    pub fn is_walkable(&self, x:i32, y:i32) -> bool {
        let idx = self.tile_idx(x, y);
        match idx {
            Some(idx) => {
                match self.tiles[idx] {
                    TileType::Floor => { return true }
                    TileType::Wall => { return false }
                }
            }

            None => {
                return false;
            }
        }
    }

    // Utility function: is a tile walkable
    pub fn is_transparent(&self, x:i32, y:i32) -> bool {
        let idx = self.tile_idx(x, y);
        match idx {
            Some(idx) => {
                match self.tiles[idx] {
                    TileType::Floor => { return false }
                    TileType::Wall => { return true }
                }
            }

            None => {
                return false;
            }
        }
    }

    pub fn is_tile_visible(&self, pos : &Point) -> bool {
        let idx = self.tile_idx(pos.x, pos.y);
        match idx {
            None => { return false; }
            Some(x) => { return self.visible[x]; }
        }
    }

    pub fn tile_description(&self, pos : &Point) -> String {
        let idx = self.tile_idx(pos.x, pos.y);
        match idx {
            None => { return "".to_string(); }
            Some(x) => { 
                if self.visible[x] {
                    match self.tiles[x] {
                        TileType::Floor => { return "Floor".to_string() }
                        TileType::Wall => { return "Wall".to_string() }
                    }
                }
            }
        }
        return "".to_string();
    }
}