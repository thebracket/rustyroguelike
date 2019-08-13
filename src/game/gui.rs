use crate ::rltk;
use crate ::rltk::Console;
use rltk::{Rltk, Point, RGB, Algorithm2D, VirtualKeyCode};
use super::{Map, TileType, State, TickType};
use std::cmp::{max, min};
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::path::Path;

pub enum ItemMenuResult { Cancel, NoResponse, Selected }

pub fn render(gs : &State, ctx : &mut Rltk, map : &Map) {
    draw_map(ctx, map);
    draw_entities(gs, ctx, map);
    draw_user_interface(gs, ctx);
    draw_mouse_info(gs, ctx, map);
    for p in gs.vfx.iter() {
        p.render(ctx);
    }
}

fn draw_map(ctx : &mut Rltk, map : &Map) {
    ctx.cls();

    let mut idx = 0;
    for y in 0 .. map.height {
        for x in 0 .. map.width {

            // You wouldn't normally make this mess - clean up!
            if map.revealed[idx] {
                if map.visible[idx] {
                    match map.tiles[idx] {
                        TileType::Floor => { ctx.print_color(x, y, RGB::named(rltk::DARK_GREEN), RGB::named(rltk::BLACK), ".") }
                        TileType::Wall => { ctx.set(x, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), decorate_wall_tile(map, Point::new(x,y))) }
                        TileType::Stairs => { ctx.print_color(x, y, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), ">") }
                    }
                } else {
                    match map.tiles[idx] {
                        TileType::Floor => { ctx.print_color(x, y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), ".") }
                        TileType::Wall => { ctx.set(x, y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), decorate_wall_tile(map, Point::new(x,y))) }
                        TileType::Stairs => { ctx.print_color(x, y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), ">") }
                    }
                }
            }

            idx += 1;
        }
    }
}

fn is_revealed_and_wall(map : &Map, coord: Point) -> bool {
    let idx = map.point2d_to_index(coord) as usize;
    map.tiles[idx] == TileType::Wall && map.revealed[idx]
}

fn decorate_wall_tile(map : &Map, coord: Point) -> u8 {
    if coord.x == 0 || coord.x == map.width || coord.y == 0 || coord.y == map.height { return 35; }
    let mut mask : u8 = 0;
    if is_revealed_and_wall(map, Point::new(coord.x, coord.y - 1)) { mask += 1; }
    if is_revealed_and_wall(map, Point::new(coord.x, coord.y + 1)) { mask += 2; }
    if is_revealed_and_wall(map, Point::new(coord.x - 1, coord.y)) { mask += 4; }
    if is_revealed_and_wall(map, Point::new(coord.x + 1, coord.y)) { mask += 8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        _ => { 35 } // We missed one?
    }
}

fn draw_entities(gs: &State, ctx: &mut Rltk, map : &Map) {
    for e in gs.entities.iter() {
            e.draw_to_map(ctx, &map);
        }
}

fn draw_user_interface(gs: &State, ctx : &mut Rltk) {
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    ctx.draw_box(1, 43, 78, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let maplvl = format!("Depth: {} ", gs.player().dungeon_level);
    ctx.print_color(3, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &maplvl);

    let health = format!(" HP: {} / {} ", gs.player().fighter.hp, gs.player().fighter.max_hp);
    ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

    ctx.draw_bar_horizontal(28, 43, 51, gs.player().fighter.hp, gs.player().fighter.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));

    let mut y = 44;
    for s in gs.log.iter() {
        ctx.print(2, y, &s.to_string());
        y += 1;
    }
}

fn draw_mouse_info(gs : &State, ctx : &mut Rltk, map: &Map) {
    let mouse_pos = ctx.mouse_pos();
    if map.is_tile_visible(Point::new(mouse_pos.0, mouse_pos.1)) {
        let mut tooltip : Vec<String> = Vec::new();

        let tile_info = map.tile_description(Point::new(mouse_pos.0, mouse_pos.1));
        tooltip.push(format!("Tile: {}", tile_info));

        for e in gs.entities.iter() {
            if e.get_position() == Point::new(mouse_pos.0, mouse_pos.1) {
                tooltip.push(e.get_tooltip_text());
            }
        }

        if !tooltip.is_empty() {
            let mut width :i32 = 0;
            for s in tooltip.iter() {
                if width < s.len() as i32 { width = s.len() as i32; }
            }
            width += 3;

            if mouse_pos.0 > 40 {
                let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
                let left_x = mouse_pos.0 - width;
                let mut y = mouse_pos.1;
                for s in tooltip.iter() {
                    ctx.print_color(left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &s.to_string());
                    let padding = (width - s.len() as i32)-1;
                    for i in 0..padding {
                        ctx.print_color(arrow_pos.x - i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
                    }
                    y += 1;
                }
                ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"->".to_string());
            } else {
                let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
                let left_x = mouse_pos.0 +3;
                let mut y = mouse_pos.1;
                for s in tooltip.iter() {
                    ctx.print_color(left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &s.to_string());
                    let padding = (width - s.len() as i32)-1;
                    for i in 0..padding {
                        ctx.print_color(left_x + s.len() as i32 + i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
                    }
                    y += 1;
                }
                ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"<-".to_string());
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn handle_item_menu<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> (ItemMenuResult, i32) {
    let count = gs.player().inventory.items.len();
    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &title.to_string());

    for (j,i) in gs.player().inventory.items.iter().enumerate() {
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 40);
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as u8);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 41);

        ctx.print(21, y, &i.name.to_string());
        y += 1;
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::Escape => { return (ItemMenuResult::Cancel, 0) }
                _ => { 
                    let selection = rltk::letter_to_option(KEY);
                    if selection > -1 && selection < gs.player().inventory.items.len() as i32 {
                        return (ItemMenuResult::Selected, selection);
                    }  
                    return (ItemMenuResult::NoResponse, 0);
                }
            }
        }
    }

    (ItemMenuResult::NoResponse, 0)
}

#[allow(non_snake_case)]
pub fn handle_equippable_menu<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> (ItemMenuResult, i32) {
    let equippable = gs.player().inventory.get_equippable_items();
    let count = equippable.len();
    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &title.to_string());

    for (j,i) in equippable.iter().enumerate() {
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 40);
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as u8);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 41);

        ctx.print(21, y, &gs.player().inventory.items[*i as usize].name.to_string());
        y += 1;
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::Escape => { return (ItemMenuResult::Cancel, 0) }
                _ => { 
                    let selection = rltk::letter_to_option(KEY);
                    if selection > -1 && selection < gs.player().inventory.items.len() as i32 {
                        return (ItemMenuResult::Selected, equippable[selection as usize]);
                    }  
                    return (ItemMenuResult::NoResponse, 0);
                }
            }
        }
    }

    (ItemMenuResult::NoResponse, 0)
}

#[allow(non_snake_case)]
pub fn handle_equipped_menu<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> (ItemMenuResult, i32) {
    let count = gs.player().inventory.equipped.len();
    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &title.to_string());

    for (j,i) in gs.player().inventory.equipped.iter().enumerate() {
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 40);
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as u8);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 41);

        ctx.print(21, y, &i.name.to_string());
        y += 1;
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::Escape => { return (ItemMenuResult::Cancel, 0) }
                _ => { 
                    let selection = rltk::letter_to_option(KEY);
                    if selection > -1 && selection < gs.player().inventory.equipped.len() as i32 {
                        return (ItemMenuResult::Selected, selection);
                    }  
                    return (ItemMenuResult::NoResponse, 0);
                }
            }
        }
    }

    (ItemMenuResult::NoResponse, 0)
}

pub fn display_game_over_and_handle_quit(ctx : &mut Rltk, gs : &mut State) {
    ctx.cls();
    ctx.print_color(33, 25, RGB::named(rltk::RED), RGB::named(rltk::BLACK), &"You are dead.".to_string());
    ctx.print_color(28, 27, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &"Press any key for the menu.".to_string());
    if let Some(_) = ctx.key { gs.game_state = TickType::MainMenu }
}

#[allow(non_snake_case)]
pub fn handle_item_targeting<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> ItemMenuResult {
    ctx.print_color(0,0, RGB::named(rltk::YELLOW), RGB::named(rltk::RED), &title.to_string());
    let mouse_tuple = ctx.mouse_pos();
    let mouse_pos = Point::new(mouse_tuple.0, mouse_tuple.1);
    let previous_mouse = gs.prev_mouse_for_targeting;

    if mouse_pos != previous_mouse && mouse_pos.x > 0 && mouse_pos.x < 79 && mouse_pos.y > 0 && mouse_pos.y < 40 { gs.target_cell = mouse_pos; }

    if gs.target_cell.x < 1 { gs.target_cell.x = 1; }
    if gs.target_cell.x > 79 { gs.target_cell.x = 79; }
    if gs.target_cell.y < 1 { gs.target_cell.y = 1; }
    if gs.target_cell.y > 39 { gs.target_cell.y = 39; }

    let possible = gs.map.is_tile_visible(gs.target_cell);

    if possible {
        ctx.set_bg(gs.target_cell.x, gs.target_cell.y, RGB::named(rltk::RED));
        if ctx.left_click {
            return ItemMenuResult::Selected;
        }
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::Escape => { return ItemMenuResult::Cancel }
                VirtualKeyCode::Return => { if possible { return ItemMenuResult::Selected } }
                VirtualKeyCode::Space => { if possible { return ItemMenuResult::Selected } }
                VirtualKeyCode::Left => { gs.target_cell.x = max(gs.target_cell.x-1, 1) }
                VirtualKeyCode::Right => { gs.target_cell.x = min(gs.target_cell.x+1, 79) }
                VirtualKeyCode::Up => { gs.target_cell.y = max(gs.target_cell.y-1, 1) }
                VirtualKeyCode::Down => { gs.target_cell.y = min(gs.target_cell.y+1, 40) }
                VirtualKeyCode::Numpad4 => { gs.target_cell.x = max(gs.target_cell.x-1, 1) }
                VirtualKeyCode::Numpad6 => { gs.target_cell.x = min(gs.target_cell.x+1, 79) }
                VirtualKeyCode::Numpad8 => { gs.target_cell.y = max(gs.target_cell.y-1, 1) }
                VirtualKeyCode::Numpad2 => { gs.target_cell.y = min(gs.target_cell.y+1, 40) }
                VirtualKeyCode::Numpad7 => { gs.target_cell = Point::new(  max(gs.target_cell.x-1, 1), max(gs.target_cell.y-1, 1) ) }
                VirtualKeyCode::Numpad9 => { gs.target_cell = Point::new(  min(gs.target_cell.x+1, 79), max(gs.target_cell.y-1, 1) ) }
                VirtualKeyCode::Numpad1 => { gs.target_cell = Point::new(  max(gs.target_cell.x-1, 1), min(gs.target_cell.y+1, 40) ) }
                VirtualKeyCode::Numpad3 => { gs.target_cell = Point::new(  min(gs.target_cell.x+1, 79), min(gs.target_cell.y+1, 40) ) }
                _ => { }
            }
        }
    }

    ItemMenuResult::NoResponse
}

const STORY_TYPES : & [& str] = &["Tales", "Sagas", "Adventures", "Anecdotes", "Fables", "Narratives"];
const STORY_NOUNS : & [& str] = &["Heroism", "Cowardice", "Vengeance", "Heroism", "Exploration", "Delving", "Dungeoneering"];

#[derive(Serialize, Deserialize)]
pub struct MenuState {
    random : Vec<usize>,
    save_exists : bool,
    current_menu_option : i32,
    backdrop : Vec<(u8, f32)>
}

impl MenuState {
    pub fn new() -> MenuState {
        let mut rng = rand::thread_rng();
        let save_exists = Path::new("./savegame.json").exists();
        let mut cmo = 1;
        if save_exists { cmo = 0; }

        let mut bd : Vec<(u8, f32)> = Vec::new();
        for _i in 0..(80*50) {
            let bg_i = rng.gen_range(0, 192);
            let bg : f32 = bg_i as f32 / 255.0;
            bd.push((rng.gen_range(32, 62) as u8, bg));
        }

        MenuState{
            random: vec![rng.gen_range(0, 6), rng.gen_range(0, 7), rng.gen_range(0, 7)],
            save_exists,
            current_menu_option : cmo,
            backdrop : bd
        }
    }
}

pub enum MainMenuResult { None, Continue, New, Quit }

#[allow(non_snake_case)]
pub fn display_main_menu(ctx : &mut Rltk, ms : &mut MenuState) -> MainMenuResult {
    let mut rng = rand::thread_rng();
    ctx.cls();

    // Backdrop
    for y in 0..50 {
        for x in 0..80 {
            let idx = (y*80)+x;
            ctx.set(x, y, RGB::from_f32(0.0, ms.backdrop[idx as usize].1, 0.0), RGB::named(rltk::BLACK), ms.backdrop[idx as usize].0);
        }
    }

    for x in 0..80 {
        for y in (1..50).rev() {
            let idx = (y * 80) + x;
            let above_idx = ((y-1) * 80) + x;
            ms.backdrop[idx] = ms.backdrop[above_idx];
            ms.backdrop[idx].1 -= 0.02;
            if ms.backdrop[idx].1 < 0.0 {
                let bg_i = rng.gen_range(0, 192);
                let bg : f32 = bg_i as f32 / 255.0;
                ms.backdrop[idx] = (rng.gen_range(32, 62) as u8, bg);
            }
        }
        let y = 0;
        let idx = (y * 80) + x;
        let bg_i = rng.gen_range(0, 192);
        let bg : f32 = bg_i as f32 / 255.0;
        ms.backdrop[idx] = (rng.gen_range(32, 62) as u8, bg);
    }

    // Header
    ctx.draw_box(15, 8, 50, 11, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK));
    ctx.print_color_centered(10, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Rusty Roguelike v1.0");
    ctx.print_color_centered(12, RGB::named(rltk::RED), RGB::named(rltk::BLACK), &format!("{} in {} and {}", STORY_TYPES[ms.random[0]], STORY_NOUNS[ms.random[1]], STORY_NOUNS[ms.random[2]]));

    // Menu render
    let mut y = 15;
    if ms.save_exists {
        if ms.current_menu_option == 0 {
            ctx.print_color_centered(y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "(C)ontinue Saved Game");
        } else {
            ctx.print_color_centered(y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "(C)ontinue Saved Game");
        }
        y += 1;
    }
    if ms.current_menu_option == 1 {
        ctx.print_color_centered(y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "(N)ew Game");
    } else {
        ctx.print_color_centered(y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "(N)ew Game");
    }
    y += 1;
    if ms.current_menu_option == 2 {
        ctx.print_color_centered(y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "(Q)uit");
    } else {
        ctx.print_color_centered(y, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "(Q)uit");
    }

    // Copyright blurb
    ctx.print_color_centered(42, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "/r/roguelikedev Roguelike Tutorial Series");
    ctx.print_color_centered(43, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "https://github.com/thebracket/rustyroguelike");
    ctx.print_color_centered(44, RGB::named(rltk::GREY), RGB::named(rltk::BLACK), "(c) 2019 Bracket Productions");

    // Keyboard input
    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::Escape => { return MainMenuResult::Quit }
                VirtualKeyCode::Q => { return MainMenuResult::Quit }
                VirtualKeyCode::N => { return MainMenuResult::New }
                VirtualKeyCode::C => { if ms.save_exists { return MainMenuResult::Continue } }
                VirtualKeyCode::Up => {
                    ms.current_menu_option -= 1;
                    if ms.save_exists && ms.current_menu_option < 0 { ms.current_menu_option = 2 }
                    if (!ms.save_exists) && ms.current_menu_option < 1 { ms.current_menu_option = 1 }
                }
                VirtualKeyCode::Down => {
                    ms.current_menu_option += 1;
                    if ms.save_exists && ms.current_menu_option > 2 { ms.current_menu_option = 0 }
                    if (!ms.save_exists) && ms.current_menu_option > 2 { ms.current_menu_option = 1 }
                }
                VirtualKeyCode::Return => {
                    match ms.current_menu_option {
                        0 => { return MainMenuResult::Continue }
                        1 => { return MainMenuResult::New }
                        2 => { return MainMenuResult::Quit }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    MainMenuResult::None
}

#[allow(non_snake_case)]
pub fn handle_level_up(ctx : &mut Rltk, gs : &mut State) {

    ctx.draw_box(10, 8, 60, 18, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color_centered(10, RGB::named(rltk::WHITE), RGB::named(rltk::RED), &format!("Congratulations, you are now level {}!", gs.player().level));
    ctx.print_color_centered(12, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Your experience has improved your battle prowess.");
    ctx.print_color_centered(13, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Select one of the following to improve:");
    ctx.print_color_centered(15, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "(A) Give me more hit points.");
    ctx.print_color_centered(16, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "(B) I'd like to do more damage.");

    // Keyboard input
    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                VirtualKeyCode::A => { 
                    gs.player_mut().fighter.max_hp += 10;
                    gs.player_mut().fighter.hp = gs.player().fighter.max_hp;
                    gs.game_state = TickType::PlayersTurn;
                }
                VirtualKeyCode::B => { 
                    gs.player_mut().fighter.power += 1;
                    gs.game_state = TickType::PlayersTurn;
                }
                _ => {}
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn display_character_info(ctx : &mut Rltk, gs : &mut State) {
    let player = gs.player();
    ctx.draw_box(10, 8, 60, 16, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color_centered(10, RGB::named(rltk::WHITE), RGB::named(rltk::RED), "Character Information");
    ctx.print_color_centered(12, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "You are not dead yet. That's something.");
    ctx.print_color_centered(13, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("You have beaten {} dungeon levels.", player.dungeon_level));
    ctx.print_color_centered(14, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("You have {} experience points, needing {} to level.", player.xp, player.xp_to_level()));
    ctx.print_color_centered(15, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("You are level {}.", player.level));
    ctx.print_color_centered(16, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("You have {} hit points, out of {}.", player.fighter.hp, player.fighter.max_hp));
    ctx.print_color_centered(17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("Your hit power is {}.", player.fighter.power));
    ctx.print_color_centered(18, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &format!("Your defense power is {}.", player.fighter.defense));

    ctx.print_color_centered(20, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Press any key to resume dungeon bashing!");

    match ctx.key {
        None => {}
        Some(_) => {
            gs.game_state = TickType::PlayersTurn;
        }
    }
}

#[allow(non_snake_case)]
pub fn display_help_info(ctx : &mut Rltk, gs : &mut State) {
    ctx.draw_box(10, 8, 60, 17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color_centered(10, RGB::named(rltk::WHITE), RGB::named(rltk::RED), "Controls");
    ctx.print_color_centered(12, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Arrow keys or NumPad keys to move.");
    ctx.print_color_centered(13, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Walk into a monster to attack it.");
    ctx.print_color_centered(14, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "NumPad 5, or W to Wait.");
    ctx.print_color_centered(15, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "G to Get an item from the ground.");
    ctx.print_color_centered(16, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "U to Use an item from your inventory.");
    ctx.print_color_centered(17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "E to Equip an item from your inventory.");
    ctx.print_color_centered(17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "R to Remove an item you are using.");
    ctx.print_color_centered(17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "D to Drop an item from your inventory.");
    ctx.print_color_centered(18, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "> to go down stairs, if you are standing on them.");
    ctx.print_color_centered(19, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "C for Character Info.");
    ctx.print_color_centered(20, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "? for this help menu. You've found this one.");
    ctx.print_color_centered(21, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "ESCAPE to save the game and quit to the menu.");

    ctx.print_color_centered(23, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Press any key to resume dungeon bashing!");

    match ctx.key {
        None => {}
        Some(_) => {
            gs.game_state = TickType::PlayersTurn;
        }
    }
}