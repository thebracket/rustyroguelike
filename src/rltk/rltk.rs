extern crate glfw;
use self::glfw::{Context, Action};
extern crate gl;

use std::sync::mpsc::Receiver;
use super::Console;
use super::GameState;
use std::time::{Instant};
use super::Point;

#[allow(non_snake_case)]
pub struct Rltk {
    pub glfw : glfw::Glfw,
    pub window : glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width_pixels : u32,
    pub height_pixels : u32,
    pub consoles : Vec<Console>,
    pub fps: f64,
    pub key : Option<i32>,
    pub mouse_pos : Point,
    pub left_click : bool,
}

#[allow(non_snake_case)]
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
        };
    }

    pub fn init_simple_console<S: ToString>(width_chars:u32, height_chars:u32, window_title: S) -> Rltk {
        let mut rltk = Rltk::init_raw(width_chars * 8, height_chars * 8, window_title);
        Console::init(width_chars, height_chars, &mut rltk);
        return rltk;
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

                glfw::WindowEvent::Key(_, KEY, Action::Press, _) => {
                    self.key = Some(KEY);
                }

                glfw::WindowEvent::Key(_, KEY, Action::Repeat, _) => {
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
}

pub fn init_simple_console<S: ToString>(width_chars:u32, height_chars:u32, window_title: S) -> Rltk {
    return Rltk::init_simple_console(width_chars, height_chars, window_title);
}