use super::color::Color;
use super::tile::Tile;
use super::shader::Shader;
use super::Rltk;
use super::point::Point;

use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;

extern crate image;
use image::GenericImage;

extern crate glfw;

#[allow(non_snake_case)]
pub struct Console {
    pub width :u32,
    pub height: u32,
    pub font_width: u8,
    pub font_height: u8,

    // Private
    tiles: Vec<Tile>,
    is_dirty: bool,

    // GL Stuff
    vertex_buffer : Vec<f32>,
    index_buffer : Vec<i32>,
    console_texture: u32,
    console_shader: Shader,
    VBO: u32,
    VAO: u32,
    EBO: u32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Console {
    pub fn init(width:u32, height:u32, ctx:&mut Rltk) {
        // Console backing init
        let num_tiles : usize = (width * height) as usize;
        let mut tiles : Vec<Tile> = Vec::with_capacity(num_tiles);
        for _i in 0..num_tiles {
            tiles.push(Tile{glyph: 0, fg: Color::white(), bg: Color::black()});
        }

        let (console_shader, VBO, VAO, EBO, texture) = Console::init_gl_for_console();
        
        let new_console = Console{
            width: width, 
            height: height, 
            console_texture: texture,
            console_shader: console_shader,
            VBO: VBO,
            VAO: VAO,
            EBO: EBO,
            tiles: tiles,
            is_dirty: true,
            font_width : 8,
            font_height : 8,
            vertex_buffer : Vec::new(),
            index_buffer : Vec::new()
        };
        ctx.consoles.push(new_console);
    }

    fn init_gl_for_console() -> (Shader, u32, u32, u32, u32) {
        let mut texture = 0;
        unsafe {
            // build and compile our shader program
            let console_shader = Shader::new("resources/4.1.texture.vs", "resources/4.1.texture.fs");

            // Generate buffers and arrays, as well as attributes.
            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

            let stride = 11 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
             // bgcolor attribute
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);
            // texture coord attribute
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, stride, (9 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(3);
            
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            // load image, create texture and generate mipmaps
            let img_orig = image::open(&Path::new("resources/terminal8x8.jpg")).expect("Failed to load texture");
            let img = img_orig.flipv();
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            return (console_shader, VBO, VAO, EBO, texture)
        };
    }

    fn push_point(vertex_buffer: &mut Vec<f32>, x:f32, y:f32, fg:&Color, bg:&Color, ux:f32, uy:f32) {
        vertex_buffer.push(x);
        vertex_buffer.push(y);
        vertex_buffer.push(0.0);
        vertex_buffer.push(fg.r);
        vertex_buffer.push(fg.g);
        vertex_buffer.push(fg.b);
        vertex_buffer.push(bg.r);
        vertex_buffer.push(bg.g);
        vertex_buffer.push(bg.b);
        vertex_buffer.push(ux);
        vertex_buffer.push(uy);
    }

    fn rebuild_vertices(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();

        let glyph_size : f32 = 1.0 / 16.0;

        let step_x : f32 = 2.0 / self.width as f32;
        let step_y : f32 = 2.0 / self.height as f32;

        let mut index_count : i32 = 0;
        let mut screen_y : f32 = -1.0;
        for y in 0 .. self.height {
            let mut screen_x : f32 = -1.0;
            for x in 0 .. self.width {
                let fg = &self.tiles[((y * self.width) + x) as usize].fg;
                let bg = &self.tiles[((y * self.width) + x) as usize].bg;
                let glyph = self.tiles[((y * self.width) + x) as usize].glyph;
                let glyph_x = glyph % 16;
                let glyph_y = 16 - (glyph / 16);

                let glyph_left = glyph_x as f32 * glyph_size;
                let glyph_right = (glyph_x+1) as f32 * glyph_size;
                let glyph_top = glyph_y as f32 * glyph_size;
                let glyph_bottom = (glyph_y-1) as f32 * glyph_size;

                Console::push_point(&mut self.vertex_buffer, screen_x + step_x, screen_y + step_y, fg, bg, glyph_right, glyph_top);
                Console::push_point(&mut self.vertex_buffer, screen_x + step_x, screen_y, fg, bg, glyph_right, glyph_bottom);
                Console::push_point(&mut self.vertex_buffer, screen_x, screen_y, fg, bg, glyph_left, glyph_bottom);
                Console::push_point(&mut self.vertex_buffer, screen_x, screen_y + step_y, fg, bg, glyph_left, glyph_top);

                self.index_buffer.push(0 + index_count);
                self.index_buffer.push(1 + index_count);
                self.index_buffer.push(3 + index_count);
                self.index_buffer.push(1 + index_count);
                self.index_buffer.push(2 + index_count);
                self.index_buffer.push(3 + index_count);

                index_count += 4;
                screen_x += step_x;
            }
            screen_y += step_y;
        }
        
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                        (self.vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.vertex_buffer[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (self.index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.index_buffer[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);
        }
    }

    pub fn rebuild_if_dirty(&mut self) {
         if self.is_dirty {
            self.rebuild_vertices();
            self.is_dirty = false;
        }
    }

    pub fn gl_draw(&mut self) {
        unsafe {
            // bind Texture
            gl::BindTexture(gl::TEXTURE_2D, self.console_texture);

            // render container
            self.console_shader.useProgram();
            gl::BindVertexArray(self.VAO);
            gl::DrawElements(gl::TRIANGLES, (self.width * self.height * 6) as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }

/////////////////////////////// User facing stuff

    pub fn at(&self, pt:Point) -> usize {
        return (((self.height-1 - pt.y as u32) * self.width) + pt.x as u32) as usize;
    }

    pub fn cls(&mut self) {
        self.is_dirty = true;
        for tile in self.tiles.iter_mut() {
            tile.glyph = 0;
            tile.fg = Color::white();
            tile.bg = Color::black();
        }
    }

    pub fn print(&mut self, pt:Point, text:String) {
        self.is_dirty = true;
        let mut idx = self.at(pt);

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = bytes[i];
                idx += 1;
            }
        }
    }

    pub fn print_color(&mut self, pt:Point, fg:Color, bg:Color, text:String) {
        self.is_dirty = true;
        let mut idx = self.at(pt);

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = bytes[i];
                self.tiles[idx].fg.r = fg.r;
                self.tiles[idx].fg.g = fg.g;
                self.tiles[idx].fg.b = fg.b;
                self.tiles[idx].bg.r = bg.r;
                self.tiles[idx].bg.g = bg.g;
                self.tiles[idx].bg.b = bg.b;
                idx += 1;
            }
        }
    }

    pub fn set(&mut self, pt:Point, fg:Color, bg:Color, glyph:u8) {
        let idx = self.at(pt);
        if idx > 0 && idx < self.tiles.len() {
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg.r = fg.r;
            self.tiles[idx].fg.g = fg.g;
            self.tiles[idx].fg.b = fg.b;
            self.tiles[idx].bg.r = bg.r;
            self.tiles[idx].bg.g = bg.g;
            self.tiles[idx].bg.b = bg.b;
        }
    }

    pub fn set_bg(&mut self, pt:Point, bg:Color) {
        let idx = self.at(pt);
        if idx > 0 && idx < self.tiles.len() {
            self.tiles[idx].bg.r = bg.r;
            self.tiles[idx].bg.g = bg.g;
            self.tiles[idx].bg.b = bg.b;
        }
    }

    pub fn draw_box(&mut self, pt:Point, width:i32, height:i32, fg: Color, bg: Color) {
        self.set(pt, fg, bg, 218);
        self.set(Point::new(pt.x + width, pt.y), fg, bg, 191);
        self.set(Point::new(pt.x, pt.y + height), fg, bg, 192);
        self.set(Point::new(pt.x + width, pt.y + height), fg, bg, 217);
        for x in pt.x+1 .. pt.x + width {
            self.set(Point::new(x, pt.y), fg, bg, 196);
            self.set(Point::new(x, pt.y + height), fg, bg, 196);
        }
        for y in pt.y+1 .. pt.y + height {
            self.set(Point::new(pt.x, y), fg, bg, 179);
            self.set(Point::new(pt.x + width, y), fg, bg, 179);
        }
    }

    pub fn draw_bar_horizontal(&mut self, pt:Point, width:i32, n:i32, max:i32, fg:Color, bg: Color) {
        let percent = n as f32 / max as f32;
        let fill_width = (percent * width as f32) as i32;
        for x in 0..width {
            if x <= fill_width {
                self.set(Point::new(pt.x + x, pt.y), fg, bg, 178);
            } else {
                self.set(Point::new(pt.x + x, pt.y), fg, bg, 176);
            }
        }
    }

    pub fn quit(&mut self, ctx: &mut Rltk) {
        ctx.window.set_should_close(true)
    }
}
