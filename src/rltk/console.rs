use super::color::Color;
use super::tile::Tile;
use super::shader::Shader;
use super::Rltk;

use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::time::{Instant};

extern crate image;
use image::GenericImage;

extern crate glfw;
use self::glfw::{Context, Action};

#[allow(non_snake_case)]
pub struct Console {
    pub width :u32,
    pub height: u32,
    console_texture: u32,
    ourShader: Shader,
    VBO: u32,
    VAO: u32,
    EBO: u32,
    tiles: Vec<Tile>,
    ctx: Rltk,
    dirty: bool,
    pub fps: f64,
    pub key : Option<i32>
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Console {
    pub fn init(width:u32, height:u32, ctx:Rltk) -> Console {
        // Console backing init
        let num_tiles : usize = (width * height) as usize;
        let mut tiles : Vec<Tile> = Vec::with_capacity(num_tiles);
        for _i in 0..num_tiles {
            tiles.push(Tile{glyph: 0, fg: Color::white(), bg: Color::black()});
        }

        // Shader init
        let mut texture = 0;
        let (ourShader, VBO, VAO, EBO, texture) = unsafe {
            // build and compile our shader program
            // ------------------------------------
            let ourShader = Shader::new(
                "resources/4.1.texture.vs",
                "resources/4.1.texture.fs");

            // set up vertex data (and buffer(s)) and configure vertex attributes
            // ------------------------------------------------------------------
            // HINT: type annotation is crucial since default for float literals is f64
            let vertices: [f32; 44] = [
                // positions       // colors        // texture coords
                0.5,  0.5, 0.0,   1.0, 0.0, 0.0,  0.0,0.0,0.0,   1.0, 1.0, // top right
                0.5, -0.5, 0.0,   0.0, 1.0, 0.0,  0.0,0.0,0.0,  1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0,   0.0, 0.0, 1.0, 0.0,0.0,0.0,   0.0, 0.0, // bottom left
                -0.5,  0.5, 0.0,   1.0, 1.0, 0.0, 0.0,0.0,0.0,   0.0, 1.0  // top left
            ];
            let indices = [
                0, 1, 3,  // first Triangle
                1, 2, 3   // second Triangle
            ];
            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &indices[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);

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

            (ourShader, VBO, VAO, EBO, texture)
        };

        return Console{
            width: width, 
            height: height, 
            console_texture: texture,
            ourShader: ourShader,
            VBO: VBO,
            VAO: VAO,
            EBO: EBO,
            tiles: tiles,
            ctx: ctx,
            dirty: true,
            fps: 0.0,
            key: None
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
        let mut vertex_buffer : Vec<f32> = Vec::new();
        let mut index_buffer : Vec<i32> = Vec::new();

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

                Console::push_point(&mut vertex_buffer, screen_x + step_x, screen_y + step_y, fg, bg, glyph_right, glyph_top);
                Console::push_point(&mut vertex_buffer, screen_x + step_x, screen_y, fg, bg, glyph_right, glyph_bottom);
                Console::push_point(&mut vertex_buffer, screen_x, screen_y, fg, bg, glyph_left, glyph_bottom);
                Console::push_point(&mut vertex_buffer, screen_x, screen_y + step_y, fg, bg, glyph_left, glyph_top);

                index_buffer.push(0 + index_count);
                index_buffer.push(1 + index_count);
                index_buffer.push(3 + index_count);
                index_buffer.push(1 + index_count);
                index_buffer.push(2 + index_count);
                index_buffer.push(3 + index_count);

                index_count += 4;
                screen_x += step_x;
            }
            screen_y += step_y;
        }
        
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                        (vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertex_buffer[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &index_buffer[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);
        }
    }

    pub fn main_loop(&mut self, callback: &mut FnMut(&mut Console)) {
        let now = Instant::now();
        let mut prev_seconds = now.elapsed().as_secs();
        let mut frames = 0;

        while !self.ctx.window.should_close() {
            let now_seconds = now.elapsed().as_secs();
            frames += 1;

            if now_seconds > prev_seconds {
                self.fps = frames as f64 / (now_seconds - prev_seconds) as f64;
                prev_seconds = now_seconds;
                frames = 0;
            }

            // events
            // -----
            self.process_events();
            callback(self);

            // Console structure - doesn't really have to be every frame...
            if self.dirty {
                self.rebuild_vertices();
                self.dirty = false;
            }

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // bind Texture
                gl::BindTexture(gl::TEXTURE_2D, self.console_texture);

                // render container
                self.ourShader.useProgram();
                gl::BindVertexArray(self.VAO);
                gl::DrawElements(gl::TRIANGLES, (self.width * self.height * 6) as i32, gl::UNSIGNED_INT, ptr::null());
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            self.ctx.window.swap_buffers();
            self.ctx.glfw.poll_events();
        }
    }

    pub fn process_events(&mut self) {
        self.key = None;
        for (_, event) in glfw::flush_messages(&self.ctx.events) {

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }

                glfw::WindowEvent::Key(_, KEY, Action::Press, _) => {
                    self.key = Some(KEY);
                }
                
                _ => { }
            }
        }
    }

/////////////////////////////// User facing stuff

    pub fn at(&self, x:u32, y:u32) -> usize {
        return (((self.height-1 - y) * self.width) + x) as usize;
    }

    pub fn cls(&mut self) {
        self.dirty = true;
        for tile in self.tiles.iter_mut() {
            tile.glyph = 0;
        }
    }

    pub fn print(&mut self, x:u32, y:u32, text:String) {
        self.dirty = true;
        let mut idx = self.at(x, y);

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = bytes[i];
                idx += 1;
            }
        }
    }

    pub fn print_color(&mut self, x:u32, y:u32, fg:Color, bg:Color, text:String) {
        self.dirty = true;
        let mut idx = self.at(x, y);

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

    pub fn set(&mut self, x:u32, y:u32, fg:Color, bg:Color, glyph:u8) {
        let idx = self.at(x, y);
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

    pub fn quit(&mut self) {
        self.ctx.window.set_should_close(true)
    }
}
