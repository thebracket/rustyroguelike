Continuing to learn Rust by implementing the tutorial and the bits of `libtcod` that I need!

Here is the git repo: [Rusty Roguelike](https://github.com/thebracket/rustyroguelike). If you have any interest in Rust, feel free to crib whatever you like from it. I'm a newbie to the language, so it's probably not great, idiomatic, or etc.

I've gone quite a bit further than part 2 (I'm in part 6 at time of writing), but if you [browse to this commit](https://github.com/thebracket/rustyroguelike/tree/e5cbfa830ee415cb594f5bc4d95b7997235768c1), it's right at the end of week 2 with a bit of week 3 mixed in.

So the map with placement [looks like this](https://raw.githubusercontent.com/thebracket/rustyroguelike/e5cbfa830ee415cb594f5bc4d95b7997235768c1/resources/RustyRoguelike2.gif).

I've also made some significant progress on the library side of things. I figured out enough about traits to reduce the main file boilerplate to:

    fn main() {
        let mut gs = game::State::new();
        let mut rltk = rltk::init_with_simple_console(80, 50, "Rusty Roguelike");
        rltk.main_loop(&mut gs);
    }

The key is that gamestate now implements a trait, defined in RLTK:

    pub trait GameState {
        fn tick(&mut self, ctx : &mut Rltk);
    }

This is pretty cool. So now, *any* type that implements that can be passed to the main loop to provide a tick function. The clunky closure/lambda stuff is gone, making for a much cleaner setup. I used the same system to allow the map type to expose some information about the map, without RLTK having to know how the map works; in particular, a `get_available_exits` function.

On the library side, I've achieved (and will talk about in subsequent weeks as they come up):

* Traits that provide point-to-index and index-to-point conversions for map access (so you can stride your arrays however you like).
* Geometry functions that can provide distance and squared distance between two points (basic Euclid).
* Field of view implementation that doesn't know about your map format (brute force).
* Dijkstra flow map generation for path-finding.
* A-star path generation.
* Mouse support.

Implementing the map for this week was pretty much a 1:1 port of the tutorial. The algorithm was pretty easy to port, no major issues there. I did return a vector or rooms, rather than keep it around - and then iterate the rooms (picking a center point) for mob placement. Mobs are a basic vector. I *didn't* put the player in the mobs list - so it's not a really generic list at this point (I haven't quite figured out how to do that efficiently in Rust, yet).