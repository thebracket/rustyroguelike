use crate::rltk;
use rltk::{Rltk, Point, Color};
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
}

fn draw_map(ctx : &mut Rltk, map : &Map) {
    let console = &mut ctx.con();
    console.cls();

    let mut idx = 0;
    for y in 0 .. map.height {
        for x in 0 .. map.width {

            // You wouldn't normally make this mess - clean up!
            let coord = Point::new(x, y);
            if map.revealed[idx] {
                if map.visible[idx] {
                    match map.tiles[idx] {
                        TileType::Floor => { console.print_color(coord, Color::dark_green(), Color::black(), ".") }
                        TileType::Wall => { console.print_color(coord, Color::white(), Color::black(), "#") }
                        TileType::Stairs => { console.print_color(coord, Color::white(), Color::black(), ">") }
                    }
                } else {
                    match map.tiles[idx] {
                        TileType::Floor => { console.print_color(coord, Color::grey(), Color::black(), ".") }
                        TileType::Wall => { console.print_color(coord, Color::grey(), Color::black(), "#") }
                        TileType::Stairs => { console.print_color(coord, Color::grey(), Color::black(), ">") }
                    }
                }
            }

            idx += 1;
        }
    }
}

fn draw_entities(gs: &State, ctx: &mut Rltk, map : &Map) {
    for e in gs.entities.iter() {
            e.draw_to_map(ctx, &map);
        }
}

fn draw_user_interface(gs: &State, ctx : &mut Rltk) {
    let mouse_pos = ctx.mouse_pos;
    let console = &mut ctx.con();
    console.set_bg(mouse_pos, Color::magenta());
    console.draw_box(Point::new(1, 43), 78, 6, Color::white(), Color::black());

    let maplvl = format!("Depth: {} ", gs.player().dungeon_level);
    console.print_color(Point::new(3,43), Color::yellow(), Color::black(), maplvl);

    let health = format!(" HP: {} / {} ", gs.player().fighter.hp, gs.player().fighter.max_hp);
    console.print_color(Point::new(12,43), Color::yellow(), Color::black(), health);

    console.draw_bar_horizontal(Point::new(28, 43), 51, gs.player().fighter.hp, gs.player().fighter.max_hp, Color::red(), Color::black());

    let mut y = 44;
    for s in gs.log.iter() {
        console.print(Point::new(2, y), s.to_string());
        y += 1;
    }
}

fn draw_mouse_info(gs : &State, ctx : &mut Rltk, map: &Map) {
    let mouse_pos = ctx.mouse_pos;
    if map.is_tile_visible(mouse_pos) {
        let mut tooltip : Vec<String> = Vec::new();

        let tile_info = map.tile_description(mouse_pos);
        tooltip.push(format!("Tile: {}", tile_info));

        for e in gs.entities.iter() {
            if e.get_position() == mouse_pos {
                tooltip.push(e.get_tooltip_text());
            }
        }

        if !tooltip.is_empty() {
            let mut width :i32 = 0;
            for s in tooltip.iter() {
                if width < s.len() as i32 { width = s.len() as i32; }
            }
            width += 3;

            if ctx.mouse_pos.x > 40 {
                let arrow_pos = Point::new(ctx.mouse_pos.x - 2, ctx.mouse_pos.y);
                let left_x = ctx.mouse_pos.x - width;
                let mut y = ctx.mouse_pos.y;
                for s in tooltip.iter() {
                    ctx.con().print_color(Point::new(left_x, y), Color::white(), Color::grey(), format!("{}", s));
                    let padding = (width - s.len() as i32)-1;
                    for i in 0..padding {
                        ctx.con().print_color(Point::new(arrow_pos.x - i, y), Color::white(), Color::grey(), " ".to_string());
                    }
                    y += 1;
                }
                ctx.con().print_color(arrow_pos, Color::white(), Color::grey(), "->".to_string());
            } else {
                let arrow_pos = Point::new(ctx.mouse_pos.x + 1, ctx.mouse_pos.y);
                let left_x = ctx.mouse_pos.x +3;
                let mut y = ctx.mouse_pos.y;
                for s in tooltip.iter() {
                    ctx.con().print_color(Point::new(left_x, y), Color::white(), Color::grey(), format!("{}", s));
                    let padding = (width - s.len() as i32)-1;
                    for i in 0..padding {
                        ctx.con().print_color(Point::new(left_x + s.len() as i32 + i, y), Color::white(), Color::grey(), " ".to_string());
                    }
                    y += 1;
                }
                ctx.con().print_color(arrow_pos, Color::white(), Color::grey(), "<-".to_string());
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn handle_item_menu<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> (ItemMenuResult, i32) {
    let console = &mut ctx.con();
    let count = gs.player().inventory.items.len();
    let mut y = (25 - (count / 2)) as i32;
    let mut j = 0;

    console.draw_box(Point::new(15, y-2), 31, (count+3) as i32, Color::white(), Color::black());
    console.print_color(Point::new(18, y-2), Color::yellow(), Color::black(), title.to_string());

    for i in gs.player().inventory.items.iter() {
        console.set(Point::new(17, y), Color::white(), Color::black(), 40);
        console.set(Point::new(18, y), Color::yellow(), Color::black(), 97+j);
        console.set(Point::new(19, y), Color::white(), Color::black(), 41);

        console.print(Point::new(21, y), i.name.to_string());
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                glfw::Key::Escape => { return (ItemMenuResult::Cancel, 0) }
                _ => { 
                    let selection = Rltk::letter_to_option(KEY);
                    if selection > -1 && selection < gs.player().inventory.items.len() as i32 {
                        return (ItemMenuResult::Selected, selection);
                    }  
                    return (ItemMenuResult::NoResponse, 0);
                }
            }
        }
    }

    return (ItemMenuResult::NoResponse, 0);
}

pub fn display_game_over_and_handle_quit(ctx : &mut Rltk, gs : &mut State) {
    ctx.con().cls();
    ctx.con().print_color(Point::new(33, 25), Color::red(), Color::black(), "You are dead.".to_string());
    ctx.con().print_color(Point::new(28, 27), Color::white(), Color::black(), "Press any key for the menu.".to_string());
    match ctx.key {
        Some(_) => { gs.game_state = TickType::MainMenu }
        None => {}
    }
}

#[allow(non_snake_case)]
pub fn handle_item_targeting<S: ToString>(gs : &mut State, ctx: &mut Rltk, title: S) -> ItemMenuResult {
    ctx.con().print_color(Point::new(0,0), Color::yellow(), Color::red(), title.to_string());
    let mouse_pos = ctx.mouse_pos;
    let previous_mouse = gs.prev_mouse_for_targeting;

    if mouse_pos != previous_mouse && mouse_pos.x > 0 && mouse_pos.x < 79 && mouse_pos.y > 0 && mouse_pos.y < 40 { gs.target_cell = mouse_pos; }

    if gs.target_cell.x < 1 { gs.target_cell.x = 1; }
    if gs.target_cell.x > 79 { gs.target_cell.x = 79; }
    if gs.target_cell.y < 1 { gs.target_cell.y = 1; }
    if gs.target_cell.y > 39 { gs.target_cell.y = 39; }

    let possible = gs.map.is_tile_visible(gs.target_cell);

    if possible {
        ctx.con().set_bg(gs.target_cell, Color::red());
        if ctx.left_click {
            return ItemMenuResult::Selected;
        }
    }

    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                glfw::Key::Escape => { return ItemMenuResult::Cancel }
                glfw::Key::Enter => { if possible { return ItemMenuResult::Selected } }
                glfw::Key::Space => { if possible { return ItemMenuResult::Selected } }
                glfw::Key::Left => { gs.target_cell.x = max(gs.target_cell.x-1, 1) }
                glfw::Key::Right => { gs.target_cell.x = min(gs.target_cell.x+1, 79) }
                glfw::Key::Up => { gs.target_cell.y = max(gs.target_cell.y-1, 1) }
                glfw::Key::Down => { gs.target_cell.y = min(gs.target_cell.y+1, 40) }
                glfw::Key::Kp4 => { gs.target_cell.x = max(gs.target_cell.x-1, 1) }
                glfw::Key::Kp6 => { gs.target_cell.x = min(gs.target_cell.x+1, 79) }
                glfw::Key::Kp8 => { gs.target_cell.y = max(gs.target_cell.y-1, 1) }
                glfw::Key::Kp2 => { gs.target_cell.y = min(gs.target_cell.y+1, 40) }
                glfw::Key::Kp7 => { gs.target_cell = Point::new(  max(gs.target_cell.x-1, 1), max(gs.target_cell.y-1, 1) ) }
                glfw::Key::Kp9 => { gs.target_cell = Point::new(  min(gs.target_cell.x+1, 79), max(gs.target_cell.y-1, 1) ) }
                glfw::Key::Kp1 => { gs.target_cell = Point::new(  max(gs.target_cell.x-1, 1), min(gs.target_cell.y+1, 40) ) }
                glfw::Key::Kp3 => { gs.target_cell = Point::new(  min(gs.target_cell.x+1, 79), min(gs.target_cell.y+1, 40) ) }
                _ => { }
            }
        }
    }

    return ItemMenuResult::NoResponse;
}

const STORY_TYPES : &'static [&'static str] = &["Tales", "Sagas", "Adventures", "Anecdotes", "Fables", "Narratives"];
const STORY_NOUNS : &'static [&'static str] = &["Heroism", "Cowardice", "Vengeance", "Heroism", "Exploration", "Delving", "Dungeoneering"];

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

        return MenuState{
            random: vec![rng.gen_range(0, 6), rng.gen_range(0, 7), rng.gen_range(0, 7)],
            save_exists : save_exists,
            current_menu_option : cmo,
            backdrop : bd
        }
    }
}

pub enum MainMenuResult { None, Continue, New, Quit }

#[allow(non_snake_case)]
pub fn display_main_menu(ctx : &mut Rltk, ms : &mut MenuState) -> MainMenuResult {
    let mut rng = rand::thread_rng();
    let console = &mut ctx.con();
    console.cls();

    // Backdrop
    for y in 0..50 {
        for x in 0..80 {
            let idx = (y*80)+x;
            console.set(Point::new(x, y), Color::new(0.0, ms.backdrop[idx as usize].1, 0.0), Color::black(), ms.backdrop[idx as usize].0);
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
    console.draw_box(Point::new(15, 8), 50, 11, Color::green(), Color::black());
    console.print_color_centered(10, Color::white(), Color::black(), "Rusty Roguelike v1.0");
    console.print_color_centered(12, Color::red(), Color::black(), format!("{} in {} and {}", STORY_TYPES[ms.random[0]], STORY_NOUNS[ms.random[1]], STORY_NOUNS[ms.random[2]]));

    // Menu render
    let mut y = 15;
    if ms.save_exists {
        if ms.current_menu_option == 0 {
            console.print_color_centered(y, Color::yellow(), Color::black(), "(C)ontinue Saved Game");
        } else {
            console.print_color_centered(y, Color::grey(), Color::black(), "(C)ontinue Saved Game");
        }
        y += 1;
    }
    if ms.current_menu_option == 1 {
        console.print_color_centered(y, Color::yellow(), Color::black(), "(N)ew Game");
    } else {
        console.print_color_centered(y, Color::grey(), Color::black(), "(N)ew Game");
    }
    y += 1;
    if ms.current_menu_option == 2 {
        console.print_color_centered(y, Color::yellow(), Color::black(), "(Q)uit");
    } else {
        console.print_color_centered(y, Color::grey(), Color::black(), "(Q)uit");
    }

    // Copyright blurb
    console.print_color_centered(42, Color::grey(), Color::black(), "/r/roguelikedev Roguelike Tutorial Series");
    console.print_color_centered(43, Color::grey(), Color::black(), "https://github.com/thebracket/rustyroguelike");
    console.print_color_centered(44, Color::grey(), Color::black(), "(c) 2019 Bracket Productions");

    // Keyboard input
    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                glfw::Key::Escape => { return MainMenuResult::Quit }
                glfw::Key::Q => { return MainMenuResult::Quit }
                glfw::Key::N => { return MainMenuResult::New }
                glfw::Key::C => { if ms.save_exists { return MainMenuResult::Continue } }
                glfw::Key::Up => {
                    ms.current_menu_option -= 1;
                    if ms.save_exists && ms.current_menu_option < 0 { ms.current_menu_option = 2 }
                    if (!ms.save_exists) && ms.current_menu_option < 1 { ms.current_menu_option = 1 }
                }
                glfw::Key::Down => {
                    ms.current_menu_option += 1;
                    if ms.save_exists && ms.current_menu_option > 2 { ms.current_menu_option = 0 }
                    if (!ms.save_exists) && ms.current_menu_option > 2 { ms.current_menu_option = 1 }
                }
                glfw::Key::Enter => {
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

    return MainMenuResult::None;
}

#[allow(non_snake_case)]
pub fn handle_level_up(ctx : &mut Rltk, gs : &mut State) {
    let console = &mut ctx.con();

    console.draw_box(Point::new(10, 8), 60, 18, Color::white(), Color::black());
    console.print_color_centered(10, Color::white(), Color::red(), format!("Congratulations, you are now level {}!", gs.player().level));
    console.print_color_centered(12, Color::white(), Color::black(), "Your experience has improved your battle prowess.");
    console.print_color_centered(13, Color::white(), Color::black(), "Select one of the following to improve:");
    console.print_color_centered(15, Color::yellow(), Color::black(), "(A) Give me more hit points.");
    console.print_color_centered(16, Color::yellow(), Color::black(), "(B) I'd like to do more damage.");

    // Keyboard input
    match ctx.key {
        None => {}
        Some(KEY) => {
            match KEY {
                glfw::Key::A => { 
                    gs.player_mut().fighter.max_hp += 10;
                    gs.player_mut().fighter.hp = gs.player().fighter.max_hp;
                    gs.game_state = TickType::PlayersTurn;
                }
                glfw::Key::B => { 
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
    let console = &mut ctx.con();
    let player = gs.player();
    console.draw_box(Point::new(10, 8), 60, 16, Color::white(), Color::black());
    console.print_color_centered(10, Color::white(), Color::red(), "Character Information");
    console.print_color_centered(12, Color::white(), Color::black(), "You are not dead yet. That's something.");
    console.print_color_centered(13, Color::white(), Color::black(), format!("You have beaten {} dungeon levels.", player.dungeon_level));
    console.print_color_centered(14, Color::white(), Color::black(), format!("You have {} experience points, needing {} to level.", player.xp, player.xp_to_level()));
    console.print_color_centered(15, Color::white(), Color::black(), format!("You are level {}.", player.level));
    console.print_color_centered(16, Color::white(), Color::black(), format!("You have {} hit points, out of {}.", player.fighter.hp, player.fighter.max_hp));
    console.print_color_centered(17, Color::white(), Color::black(), format!("Your hit power is {}.", player.fighter.power));
    console.print_color_centered(18, Color::white(), Color::black(), format!("Your defense power is {}.", player.fighter.defense));

    console.print_color_centered(20, Color::yellow(), Color::black(), "Press any key to resume dungeon bashing!");

    match ctx.key {
        None => {}
        Some(_) => {
            gs.game_state = TickType::PlayersTurn;
        }
    }
}