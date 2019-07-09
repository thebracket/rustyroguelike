Continuing to learn Rust by implementing the tutorial and the bits of `libtcod` that I need!

Here is the git repo: [Rusty Roguelike](https://github.com/thebracket/rustyroguelike). If you have any interest in Rust, feel free to crib whatever you like from it. I'm a newbie to the language, so it's probably not great, idiomatic, or etc.

I've now finished the tutorial in Rust, and am working on turning RLTK_RS into a Rust library - complete with examples and tutorials. It'll be a bit before that's ready.

Week 3's tutorial went reasonably smoothly. I actually did parts of week 3 and 4 together, so it's a little harder to separate them than I'd like. You can browse the repo at the time when a random map is generated [here](https://github.com/thebracket/rustyroguelike/tree/c632e4c7c230fe536e01855428abf69f505f62cb). The mob stuff got mixed into a few later commits, but I did field-of-view first (trying to get my head around some Rust concepts).

The actual map generation wasn't too bad at all:

* I extended the `map` struct to implement `random_rooms_tut3`, which uses the same algorithm as the tutorial. This required that I learn the `rand` system from Rust (it works pretty well - word of warning, a new version is just coming out that changes the syntax quite a lot), and also made a `Rect` structure that implements `center` and `intersects` to assist. My implementation returns a vector of `Rect` structures for each room.
* This version then iterates the vector of rects (`Rooms`), and places one mob in the center of each room.

For mobs, I went with a super simple definition at this point:

    pub struct Mob {
        pub x : i32,
        pub y : i32,
        pub glyph: u8,
        pub fg : Color,
    }

And I simply added a `Vec<Mob>` to the game state. Since the goal is reusable code, I created a "trait" named `Renderable`. (Traits are like extension classes in other languages; you can implement a trait for most types, and then call the functions for that trait on the type. You setup a prototype/interface, and implement it on every type you want; you can also use polymorphism to store a mixed bag of structs that implement a trait - I did that a bit late in the game, more on that in coming weeks!). I also implemented `Renderable` for `Player`. So the render code simply calls `draw` on all mobs and the player. For now, the implementations are identical: put an ASCII glyph on the map in the right place, in the right color.

My collision detection is relatively primitive. When I detect a player movement request, I calculate the tile on which they will land - and ask the map if that tile is walkable. If it is, then I iterate through the `mobs` list and see if their position matches my position. If it does, the move is cancelled and a message (on the console, since we don't have a GUI yet) says "you kick the monster".

One fun discovery: structs don't have an equality operator by default, but you don't have to implement operator overloading to make it happen. You can tack `#[derive(Eq, PartialEq)]` onto a struct to have it do a member-wise comparison for equality purposes. Likewise, if you put `#[derive(Copy, Clone)]` into a stuct it no longer moves every time you assign it - it simply splats a copy out. This was especially useful for `Color` - rather than have references everywhere, it simply copies the values. Given that a `Color` is just three 32-bit floats, that's near instant since the compiler is smart enough to use registers for the purpose (and a pointer is 64-bits, so it's only slightly larger than a reference anyway).

After week 3, I used this discovery to move all my `x,y` type structures into a `Point` struct. It looks cleaner that way!