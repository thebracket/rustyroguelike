pub use self::color::Color;
mod color;

pub use self::tile::Tile;
mod tile;

pub use self::shader::Shader;
mod shader;

pub use self::console::Console;
mod console;

pub use self::point::Point;
mod point;

pub use self::fieldofview::field_of_view;
mod fieldofview;

pub use self::geometry::distance2d_squared;
pub use self::geometry::distance2d;
mod geometry;

extern crate glfw;
use self::glfw::{Context};

extern crate gl;

use std::sync::mpsc::Receiver;
use std::str;

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

        let (mut window, events) = glfw.create_window(width_pixels, height_pixels, &window_title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
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
}

