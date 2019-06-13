use crate::rltk;
use rltk::Color;
use rltk::Console;

use super::TileType;
use super::Rect;

use rand::Rng;
use std::cmp::{max, min};

const ROOM_MAX_SIZE : i32 = 10;
const ROOM_MIN_SIZE : i32 = 6;
const MAX_ROOMS : i32 = 30;

pub struct Map {
    pub tiles : Vec<TileType>
}

impl Map {
    pub fn new() -> Map {
        let mut blank_map = Vec::new();
        for _i in 0 .. (80*50) {
            blank_map.push(TileType::Wall);
        }

        return Map{tiles : blank_map};
    }

    pub fn random_rooms_tut3(&mut self) -> Vec<Rect> {
        let mut rng = rand::thread_rng();

        let mut rooms : Vec<Rect> = Vec::new();
        for _i in 1..MAX_ROOMS {
            let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let x = rng.gen_range(1, 80 - w - 1);
            let y = rng.gen_range(1, 50 - h - 1);

            let room_candidate = Rect::new(x, y, x+w, y+h);

            let mut collides = false;
            for room in rooms.iter() {
                if room_candidate.intersect(room) {
                    collides = true;
                }
            }

            if !collides {
                self.apply_room(&room_candidate);

                if rooms.len() > 0 {
                    let (new_x, new_y) = room_candidate.center();
                    let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                    if rng.gen_range(0,1)==1 {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                rooms.push(room_candidate);
            }
        }
        return rooms;
    }

    // Applies a rectangle room to the map
    fn apply_room(&mut self, rect : &Rect) {
        for y in min(rect.y1, rect.y2) .. max(rect.y1, rect.y2) {
            for x in min(rect.x1, rect.x2) .. max(rect.x1, rect.x2) {
                let idx = (y * 80) + x;
                if idx > 0 && idx < 80*50 {
                    self.tiles[idx as usize] = TileType::Floor;
                }
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) .. max(x1,x2)+1 {
            let idx = (y * 80) + x;
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) .. max(y1,y2)+1 {
            let idx = (y * 80) + x;
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn draw(&mut self, console : &mut Console) {
        console.cls();

        let mut idx = 0;
        for y in 0 .. 50 {
            for x in 0 .. 80 {
                match self.tiles[idx] {
                    TileType::Floor => { console.print_color(x, y, Color::dark_green(), Color::black(), ".".to_string()) }
                    TileType::Wall => { console.print_color(x, y, Color::white(), Color::black(), "#".to_string()) }
                }

                idx += 1;
            }
        }
    }
}