use crate::rltk;
use rltk::Color;
use rltk::Console;
use rltk::Point;
use rltk::Algorithm2D;

use super::TileType;

pub struct Map {
    pub tiles : Vec<TileType>,
    pub visible : Vec<bool>,
    pub revealed : Vec<bool>,
    pub width: i32,
    pub height: i32,
    pub blocked : Vec<bool>
}

impl Map {
    pub fn new(w:i32, h:i32) -> Map {
        let mut visible = Vec::new();
        let mut blank_map = Vec::new();
        let mut revealed = Vec::new();
        let mut blocked = Vec::new();
        for _i in 0 .. (w*h) {
            blank_map.push(TileType::Wall);
            visible.push(false);
            revealed.push(false);
            blocked.push(false);
        }

        return Map{tiles : blank_map, visible: visible, revealed: revealed, width: w, height: h, blocked: blocked};
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
        for y in 0 .. self.height {
            for x in 0 .. self.width {

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
            return Some(((y*self.width)+x) as usize);
        } else {
            return None;
        }
    }

    // Utility function: bounds checking
    fn valid_tile(&self, x:i32, y:i32) -> bool {
        return x > 0 && x < self.width-1 && y > 0 && y < self.height-1;
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

    pub fn refresh_blocked(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width) + x;
                self.blocked[idx as usize] = !self.is_walkable(x, y);
            }
        }
    }

    pub fn set_tile_blocked(&mut self, idx : i32) {
        self.blocked[idx as usize] = true;
    }

    pub fn clear_tile_blocked(&mut self, idx : i32) {
        self.blocked[idx as usize] = false;
    }

    pub fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        let idx = (y * self.width) + x;
        return !self.blocked[idx as usize];
    }

    pub fn is_tile_blocked(&self, idx: i32) -> bool {
        return self.blocked[idx as usize];
    }    
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt : Point) -> i32 {
        return (pt.y * self.width) + pt.x;
    }    

    fn index_to_point2d(&self, idx:i32) -> Point {
        return Point{ x: idx % self.width, y: idx / self.width };
    }

    fn can_see_through_tile(&self, idx: i32) -> bool {
        return self.is_transparent(idx % self.width, idx / self.width);
    }

    fn get_available_exits(&self, idx:i32) -> Vec<(i32, f32)> {
        let mut exits : Vec<(i32, f32)> = Vec::new();
        let x = idx % self.width;
        let y = idx / self.width;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-self.width, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+self.width, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-self.width)-1, 1.4)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-self.width)+1, 1.4)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+self.width)-1, 1.4)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+self.width)+1, 1.4)); }

        return exits;
    }
}