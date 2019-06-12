#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use std::sync::mpsc::Receiver;
use std::path::Path;
use std::os::raw::c_void;
use std::ptr;
use std::mem;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::str;

use gl::types::*;

use cgmath::{Matrix, Matrix4, Vector3};
use cgmath::prelude::*;

extern crate image;
use image::GenericImage;

unsafe fn glCheckError_(file: &str, line: u32) -> u32 {
    let mut errorCode = gl::GetError();
    while errorCode != gl::NO_ERROR {
        let error = match errorCode {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            _ => "unknown GL error code"
        };

        println!("{} | {} ({})", error, file, line);

        errorCode = gl::GetError();
    }
    errorCode
}

macro_rules! glCheckError {
    () => (
        glCheckError_(file!(), line!())
    )
}

pub struct Rltk {
    pub glfw : glfw::Glfw,
    pub window : glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width_pixels : u32,
    pub height_pixels : u32,
}

impl Rltk {
    fn init_raw(width_pixels:u32, height_pixels:u32, window_title: &str) -> Rltk {        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(width_pixels, height_pixels, window_title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);        

        return Rltk{glfw: glfw, window: window, events: events, width_pixels: width_pixels, height_pixels: height_pixels};
    }

    pub fn init_simple_console(width_chars:u32, height_chars:u32, window_title: String) -> Console {
        let rltk = Rltk::init_raw(width_chars * 8, height_chars * 8, &window_title);
        let con = Console::init(width_chars, height_chars, rltk);
        return con;
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
                _ => {}
            }
        }
    }
}

pub struct Console {
    pub width :u32,
    pub height: u32,
    pub console_texture: u32,
    pub ourShader: Shader,
    pub VBO: u32,
    pub VAO: u32,
    pub EBO: u32,
    pub tiles: Vec<Tile>,
    ctx: Rltk
}

pub struct Tile {
    pub glyph: u8
}

impl Console {
    pub fn init(width:u32, height:u32, ctx:Rltk) -> Console {
        // Console backing init
        let num_tiles : usize = (width * height) as usize;
        let mut tiles : Vec<Tile> = Vec::with_capacity(num_tiles);
        for _i in 0..num_tiles {
            tiles.push(Tile{glyph: 0});
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
            let vertices: [f32; 32] = [
                // positions       // colors        // texture coords
                0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
                0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
                -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
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

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
            // texture coord attribute
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);
            
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
            ctx: ctx
        };
    }

    fn push_point(vertex_buffer: &mut Vec<f32>, x:f32, y:f32, r:f32, g:f32, b:f32, ux:f32, uy:f32) {
        vertex_buffer.push(x);
        vertex_buffer.push(y);
        vertex_buffer.push(0.0);
        vertex_buffer.push(r);
        vertex_buffer.push(g);
        vertex_buffer.push(b);
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
                let glyph = self.tiles[((y * self.width) + x) as usize].glyph;
                let glyph_x = glyph % 16;
                let glyph_y = 16 - (glyph / 16);

                let glyph_left = glyph_x as f32 * glyph_size;
                let glyph_right = (glyph_x+1) as f32 * glyph_size;
                let glyph_top = glyph_y as f32 * glyph_size;
                let glyph_bottom = (glyph_y-1) as f32 * glyph_size;

                Console::push_point(&mut vertex_buffer, screen_x + step_x, screen_y + step_y, 1.0, 1.0, 1.0, glyph_right, glyph_top);
                Console::push_point(&mut vertex_buffer, screen_x + step_x, screen_y, 1.0, 1.0, 1.0, glyph_right, glyph_bottom);
                Console::push_point(&mut vertex_buffer, screen_x, screen_y, 1.0, 1.0, 1.0, glyph_left, glyph_bottom);
                Console::push_point(&mut vertex_buffer, screen_x, screen_y + step_y, 1.0, 1.0, 1.0, glyph_left, glyph_top);

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
            glCheckError!();
            gl::BufferData(gl::ARRAY_BUFFER,
                        (vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertex_buffer[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);
            glCheckError!();

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            glCheckError!();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &index_buffer[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);
            glCheckError!();
        }
    }

    pub fn main_loop(&mut self, callback: fn()) {
        while !self.ctx.window.should_close() {
            // events
            // -----
            self.ctx.process_events();
            callback();

            // Console structure - doesn't really have to be every frame...
            self.rebuild_vertices();

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

    pub fn cls(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.glyph = 0;
        }
    }

    pub fn print(&mut self, x:u32, y:u32, text:String) {
        let mut idx : usize = (((self.height-1 - y) * self.width) + x) as usize;

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            self.tiles[idx].glyph = bytes[i];
            idx += 1;
        }
    }
}

pub struct Shader {
    pub ID: u32,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
#[allow(dead_code)]
impl Shader {
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile = File::open(vertexPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile = File::open(fragmentPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, "FRAGMENT");
            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.ID = ID;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self) {
        gl::UseProgram(self.ID)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(gl::GetUniformLocation(self.ID, name.as_ptr()), 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.ID, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.ID, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn checkCompileErrors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }
        }

    }

    /// Only used in 4.9 Geometry shaders - ignore until then (shader.h in original C++)
    pub fn with_geometry_shader(vertexPath: &str, fragmentPath: &str, geometryPath: &str) -> Self {
        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile = File::open(vertexPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile = File::open(fragmentPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut gShaderFile = File::open(geometryPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", geometryPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        let mut geometryCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");
        gShaderFile
            .read_to_string(&mut geometryCode)
            .expect("Failed to read geometry shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();
        let gShaderCode = CString::new(geometryCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, "FRAGMENT");
            // geometry shader
            let geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &gShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            shader.checkCompileErrors(geometry, "GEOMETRY");

            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::AttachShader(ID, geometry);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);
            shader.ID = ID;
        }

        shader
    }
}