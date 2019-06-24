extern crate glfw;
use self::glfw::{Context, Action};
extern crate gl;

use std::sync::mpsc::Receiver;
use super::Console;
use super::GameState;
use std::time::{Instant};
use super::Point;
use super::Font;
pub use glfw::Key;

#[allow(non_snake_case)]
pub struct Rltk {
    pub glfw : glfw::Glfw,
    pub window : glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width_pixels : u32,
    pub height_pixels : u32,
    pub consoles : Vec<Console>,
    pub fps: f64,
    pub key : Option<Key>,
    pub mouse_pos : Point,
    pub left_click : bool,
    pub active_console: usize
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Rltk {
    fn init_raw<S: ToString>(width_pixels:u32, height_pixels:u32, window_title: S) -> Rltk {        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(width_pixels, height_pixels, &window_title.to_string(), glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);        

        return Rltk{
            glfw: glfw, 
            window: window, 
            events: events, 
            width_pixels: width_pixels, 
            height_pixels: height_pixels,
            consoles: Vec::new(),
            fps: 0.0,
            key: None,
            mouse_pos: Point::new(0,0),
            left_click: false,
            active_console: 0
        };
    }

    pub fn init_simple_console(&mut self, width_chars:u32, height_chars:u32, font : Font) -> usize {
        return Console::init(width_chars, height_chars, self, font);
    }

    pub fn main_loop(&mut self, gamestate: &mut GameState) {
        let now = Instant::now();
        let mut prev_seconds = now.elapsed().as_secs();
        let mut frames = 0;

        while !self.window.should_close() {
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
            gamestate.tick(self);

            // Console structure - doesn't really have to be every frame...
            for cons in self.consoles.iter_mut() {
                cons.rebuild_if_dirty();
            }         

            // Clear the screen
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for cons in self.consoles.iter_mut() {
                cons.gl_draw();
            } 

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn process_events(&mut self) {
        self.key = None;
        self.left_click = false;
        for (_, event) in glfw::flush_messages(&self.events) {

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }

                glfw::WindowEvent::Key(KEY, _, Action::Press, _) => {
                    self.key = Some(KEY);
                }

                glfw::WindowEvent::Key(KEY, _, Action::Repeat, _) => {
                    self.key = Some(KEY);
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos.x = (x / 8.0) as i32;
                    self.mouse_pos.y = (y / 8.0) as i32;
                }

                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Press, _) => {
                    self.left_click = true;
                }
                
                _ => { }
            }
        }
    }

    pub fn quit(&mut self) {
        self.window.set_should_close(true)
    }

    pub fn set_active_console(&mut self, con : usize) {
        self.active_console = con;
    }

    pub fn con(&mut self) -> &mut Console {
        return &mut self.consoles[self.active_console];
    }

    pub fn letter_to_option(key : glfw::Key) -> i32 {
        match key {
            glfw::Key::A => { return 0; }
            glfw::Key::B => { return 1; }
            glfw::Key::C => { return 2; }
            glfw::Key::D => { return 3; }
            glfw::Key::E => { return 4; }
            glfw::Key::F => { return 5; }
            glfw::Key::G => { return 6; }
            glfw::Key::H => { return 7; }
            glfw::Key::I => { return 8; }
            glfw::Key::J => { return 9; }
            glfw::Key::K => { return 10; }
            glfw::Key::L => { return 11; }
            glfw::Key::M => { return 12; }
            glfw::Key::N => { return 13; }
            glfw::Key::O => { return 14; }
            glfw::Key::P => { return 15; }
            glfw::Key::Q => { return 16; }
            glfw::Key::R => { return 17; }
            glfw::Key::S => { return 18; }
            glfw::Key::T => { return 19; }
            glfw::Key::U => { return 20; }
            glfw::Key::V => { return 21; }
            glfw::Key::W => { return 22; }
            glfw::Key::X => { return 23; }
            glfw::Key::Y => { return 24; }
            glfw::Key::Z => { return 25; }
            _ => {return -1; }
        }
    }
}

#[allow(dead_code)]
pub fn init_no_console<S: ToString>(width_px:u32, height_px:u32, window_title: S) -> Rltk {
    let rltk = Rltk::init_raw(width_px, height_px, window_title);
    return rltk;
}

#[allow(dead_code)]
pub fn init_with_simple_console<S: ToString>(width_chars:u32, height_chars:u32, window_title: S) -> Rltk {
    let font = Font{ bitmap_file : "resources/terminal8x8.jpg".to_string(), width: 8, height: 8, render_background: true };
    let mut rltk = Rltk::init_raw(width_chars * 8, height_chars * 8, window_title);
    let con_no = rltk.init_simple_console(width_chars, height_chars, font);
    rltk.set_active_console(con_no);

    return rltk;
}

#[allow(dead_code)]
pub fn add_console(width_chars:u32, height_chars:u32, font : Font, rltk : &mut Rltk, make_active: bool) -> usize {
    let con_no = rltk.init_simple_console(width_chars, height_chars, font);
    if make_active { rltk.set_active_console(con_no); }
    return con_no;
}