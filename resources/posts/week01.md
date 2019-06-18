Since I've written a few roguelikes before (see my incredibly verbose Sharing Saturday posts!), I thought I'd use this as an excuse to learn Rust. Rather than use a `libtcod` port, I figured I'd write that, too. I have invented many wheels of varying degrees of roundness!

Here is the git repo: [Rusty Roguelike](https://github.com/thebracket/rustyroguelike). If you have any interest in Rust, feel free to crib whatever you like from it. I'm a newbie to the language, so it's probably not great, idiomatic, or etc.

I've gone a little further than this week, but you [browse to this commit](https://github.com/thebracket/rustyroguelike/tree/7000a15e6ccd96e430d32f51fef1dabf208ebdb8), it's right at the end of week 1. 2 major achievements:

* [Hello Rusty World](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/resources/RustHelloWorld2.JPG)
* [Move an @ around](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/resources/RustyRoguelike.gif)

So, how did I get this far? This was my second ever Rust project (the first was throwing spaghetti at the wall, from console "hello world" to fibonacci sequences and threaded prime number generation; the stuff I do in every language to see if I have the slightest feel for how to plug it together) - so a lot of trial and error.

* Creating a project was pretty simple with `cargo new rustyrogulike --bin`. This did more than I expected; it made a project skeleton (which you can test with `cargo build`, `cargo run`, `cargo check` and similar), made a `git` repo (so I just had to set the upstream to point it at the public Github). This gave me a canonical "hello world" on the console.
* I'm comfortable with `glfw` from Nox Futura, so I figured I'd use it as a base. Getting the support for it was as easy as adding a few lines to `cargo.toml` (`cgmath = "0.16.1" glfw = "0.29.0" glm = "0.2.3" gl = "0.12.0" image = "0.19.0"`). One `cargo check` later and everything downloaded and compiled. Nice.
* I swear by the tutorials on [https://learnopengl.com/](https://learnopengl.com/), so I was thrilled to see a [Rust port](https://github.com/bwasty/learn-opengl-rs). I spent a bit of time messing around getting a colored triangle onto the screen.
* I figured out how to make a module! Create a folder named `rltk` (Roguelike Toolkit - the name of my C++ project that provides roguelike infrastructure), make a `mod.rs` file in it and put the module in there. I still didn't like having everything in one big file, so I broke it out into smaller ones. This proved troublesome. The magic incantation turned out to be that the `mod.rs` file had to include some lines for each file: `pub use self::color::Color` (for example; that exports my `Color` structure from the `color.rs` file without ridiculously huge namespace names) and `mod color` (to tell it to use the file).
* I ended up with the `Rltk` structure that talks to OpenGL and gives me a window, and the `Console` structure that builds an array of glyph/color pairs, exposes some helpers like `cls` and `print`, and wraps the functionality to build vertex/index buffers describing my console in GL terms and with appropriate texture coordinates to output the characters. It also uses the `image` crate to load the terminal. I ran into a gotcha: loading a `png` file didn't work properly, so the font is a `jpg`.
* This got me to the point of the "hello world" gif above.
* I also spent a bit of time working on separating the boilerplate required to get things running (in `main.rs`) from the game logic (in `game.rs`). More on that below.
* Handling keyboard input brought me into the wonderful world of *pattern matching*. This is probably my favorite feature of Rust, so far. The `glfw::flush_messages` command returns a big list of variants (union types) for events that might come from the OS/library. You loop through them and `match event` on each of them. The wildcard matching is *awesome*. so `glfw::WindowEvent::Key(_, KEY, Action::Press, _) => { self.key = Some(KEY); }` matches events that are a `Key` event, with the type set to `Press`. The `_` means "I don't care what you put in there", and the `KEY` means "fill a variable named KEY with whatever is in this slot". At this point, I just save what key was pressed - but wrapped in `Some`. "Key" is an *option* - so it either has `None` or `Some(data)`. That's similar to a C++ optional, but a bit easier to read (and saves a bool, I guess).

So, the *main loop*. Unlike other languages, if you try and use mutable (changeable) global variables, Rust will complain bitterly and make you write the dreaded "unsafe" everywhere you use them. That's because it can't guaranty that threads will treat it properly. I'm not using threads, but Rust doesn't care about such trivialities. I also wanted to make the `rltk` portion reusable (maybe get to the point that a Rust version of RLTK is available), so it was important to me that the `rltk` library not know anything about how the actual game works.

So I ended up with the following boilerplate:

    mod rltk;
    mod game;
    
    fn main() {
        let mut gs = game::State::new();
        let mut console = rltk::Rltk::init_simple_console(80, 50, "Hello World".to_string());
        let mut tick_func = |console : &mut rltk::Console| {
            gs.tick(console);
        };
        console.main_loop(&mut tick_func);
    }

That's actually more boilerplate than I'd like, but it works. It starts by saying "I'm going to use the modules rltk and game" (the `mod` statements). The main function initialises `gs`, my game state. Then it initializes an 80x50 console with "Hello World" as the title. The *ugly* `let mut tick_func` stuff is a *closure* - or a lambda in other languages. It defines `tick_func` to be a special function that captures its surrounding environment - in this case the game state. So when I call the `main_loop` function, it takes that as a parameter, and calls back into my code. This was a bit of a dance, and requried learning a lot of gotchas - but it works.

So on each tick, the console calls my game's `tick` function, and - if anything changed - redraws the console. The tick function simply calls code to draw the map, draw an `@` at the player's location, and matches on the `key` variable I talked about above to see if I've used cursor keys - and moves the player if I have.

Hope this helps someone!