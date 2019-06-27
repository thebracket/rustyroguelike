use super::{ Map, Rect, TileType, Mob, Item };
use rand::Rng;
use std::cmp::{max, min};

const ROOM_MAX_SIZE : i32 = 10;
const ROOM_MIN_SIZE : i32 = 6;
const MAX_ROOMS : i32 = 30;
pub const MAX_MOBS_PER_ROOM : i32 = 8;
pub const MAX_ITEMS_PER_ROOM : i32 = 2;

pub fn random_rooms_tut3(map : &mut Map) -> Vec<Rect> {
    let mut rng = rand::thread_rng();

    let mut rooms : Vec<Rect> = Vec::new();
    for _i in 1..MAX_ROOMS {
        let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let x = rng.gen_range(1, map.width - w - 1);
        let y = rng.gen_range(1, map.height - h - 1);

        let room_candidate = Rect::new(x, y, x+w, y+h);

        let mut collides = false;
        for room in rooms.iter() {
            if room_candidate.intersect(room) {
                collides = true;
            }
        }

        if !collides {
            apply_room(map, &room_candidate);

            if rooms.len() > 0 {
                let (new_x, new_y) = room_candidate.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                if rng.gen_range(0,1)==1 {
                    apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                }
            }

            rooms.push(room_candidate);
        }
    }
    return rooms;
}

// Applies a rectangle room to the map
fn apply_room(map : &mut Map, rect : &Rect) {
    for y in min(rect.y1, rect.y2) .. max(rect.y1, rect.y2) {
        for x in min(rect.x1, rect.x2) .. max(rect.x1, rect.x2) {
            let idx = (y * map.width) + x;
            if idx > 0 && idx < map.width*map.height {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1:i32, x2:i32, y:i32) {
    for x in min(x1,x2) .. max(x1,x2)+1 {
        let idx = (y * map.width) + x;
        if idx > 0 && idx < map.width*map.height {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1:i32, y2:i32, x:i32) {
    for y in min(y1,y2) .. max(y1,y2)+1 {
        let idx = (y * map.width) + x;
        if idx > 0 && idx < map.width*map.height {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

pub fn spawn_mobs(rooms: &Vec<Rect>) -> Vec<Mob> {
    let mut rng = rand::thread_rng();
    let mut mobs : Vec<Mob> = Vec::new();
    for i in 1 .. rooms.len() {
        let number_of_mobs = rng.gen_range(1, MAX_MOBS_PER_ROOM+1);
        if number_of_mobs > 0 {
            for _mobn in 1 .. number_of_mobs {
                let mob_x = rng.gen_range(rooms[i].x1+1, rooms[i].x2-1);
                let mob_y = rng.gen_range(rooms[i].y1+1, rooms[i].y2-1);

                let mut found = false;
                for existing_mob in mobs.iter() {
                    if existing_mob.position.x == mob_x && existing_mob.position.y == mob_y {
                        found = true;
                    }
                }

                if !found {
                    let mob = Mob::new_random(mob_x, mob_y);
                    mobs.push(mob);
                }
            }
        }
    }
    return mobs;
}

pub fn spawn_items(rooms: &Vec<Rect>, mobs: &Vec<Mob>) -> Vec<Item> {
    let mut rng = rand::thread_rng();
    let mut items : Vec<Item> = Vec::new();

    for i in 1 .. rooms.len() {
        let number_of_items = rng.gen_range(1, MAX_ITEMS_PER_ROOM+1);
        if number_of_items > 0 {
            for _itemn in 1 .. number_of_items {
                let item_x = rng.gen_range(rooms[i].x1+1, rooms[i].x2-1);
                let item_y = rng.gen_range(rooms[i].y1+1, rooms[i].y2-1);

                let mut found = false;
                for existing_mob in mobs.iter() {
                    if existing_mob.position.x == item_x && existing_mob.position.y == item_y {
                        found = true;
                    }
                }

                if !found {
                    let item = Item::new_random(item_x, item_y);
                    items.push(item);
                }
            }
        }
    }

    return items;
}