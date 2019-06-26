use crate::rltk;
use rltk::Rltk;
use rltk::Point;
use rltk::Color;
use super::Map;
use super::TileType;
use super::State;

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
                        TileType::Floor => { console.print_color(coord, Color::dark_green(), Color::black(), ".".to_string()) }
                        TileType::Wall => { console.print_color(coord, Color::white(), Color::black(), "#".to_string()) }
                    }
                } else {
                    match map.tiles[idx] {
                        TileType::Floor => { console.print_color(coord, Color::grey(), Color::black(), ".".to_string()) }
                        TileType::Wall => { console.print_color(coord, Color::grey(), Color::black(), "#".to_string()) }
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
        let health = format!(" HP: {} / {} ", gs.player().fighter.hp, gs.player().fighter.max_hp);
        console.print_color(Point::new(3,43), Color::yellow(), Color::black(), health);

        console.draw_bar_horizontal(Point::new(20, 43), 59, gs.player().fighter.hp, gs.player().fighter.max_hp, Color::red(), Color::black());

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
