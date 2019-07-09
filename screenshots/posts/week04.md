My continuing adventure to boldly go where quite a few Rust programmers have been before - and learn a lot doing it. :-)

The repo for my Rust implementation of the tutorial is here: [https://github.com/thebracket/rustyroguelike](https://github.com/thebracket/rustyroguelike). As I've mentioned before, I've actually completed the tutorial now - so I'll write a bit about my implementation of tutorials 6 and 7.

*Field of View Extension*

I started out by extending my *Field of View* code to also give mobs a visibility list. It only updates when they move (lists tiles they can see, rather than contents), and is fast enough that calculating them isn't a big resource drain. I did tweak the implementation a bit to pre-allocate space rather than dynamically resizing vectors, which reduced the amount of work to be done. Then I gave mobs an FoV, and for their AI tick it simply checks the FoV list to see if the player is in it - and if they are, the mob activates.

*Path Finding*

The mob simply paths straight towards the player. I first implemented this with Dijkstra flow maps; I like to implement these whenever I'm learning a new language: they are *really* useful, really easy to get working on the basic level (with lots of room for improvement), and conceptually a lot easier than A-Star. I *highly* recommend [this article](http://www.roguebasin.com/index.php?title=The_Incredible_Power_of_Dijkstra_Maps) for reasons why you should implement them, too!

Anyway, the basic implementation went well: pass in an array of "starting points" (distance 0 spots), it iterates the list running a search on each (only applying depth if the new depth is shorter than the old one) using the time-tested open/closed list. I did learn that Rust's `HashMap` implementation could use some work - it was faster to search a small vector than to use set membership for the closed list.

It worked well, and mobs [chase the player](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/screenshots/RustyRoguelike5.gif).

I then went ahead and implemented A-Star anyway. I basically ported the version from *One Knight in the Dungeon* over, and it also works well. Speed wise, it's about the same as the C++ version.

*Fighters*

The tutorial makes a point of putting combat stuff in the *Fighter* component, so I did the same. I was surprised to discover that traits (Rust's polymorphism) don't do data members, so you end up implementing getters and setters. That felt like a step backwards, but ok. I also setup a `Combat` trait, and put the ability to bash things in there. This was pretty straight forward, and worked decently enough.

*User Interface*

I extended RLTK to include box drawing, horizontal/vertical progress bars (that work great for health bars) and a console. Since I was already passing text results around, it was a pretty straightforward task to put them into UI elements instead of sending them to the text console. The [results are quite nice](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/screenshots/RustyRoguelike6.gif).

I really wasn't enjoying my tooltips, so I hacked together a quick Cogmind-like setup (only less sophisticated and not as pretty). [Not bad for 10 minutes](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/screenshots/RustyRoguelike7.gif). It's very primitive: it goes to the left or right of the cursor based on absolute screen position (so if you are in the right half of the screen, it goes left - and vice versa).

I also went slightly out of order and [put a main menu in place](https://raw.githubusercontent.com/thebracket/rustyroguelike/master/screenshots/RustyRoguelike9.gif). I didn't get to save/load until later (the screenshot is from a slightly later build), but it felt like a good time to worry about look/feel. The Matrix effect isn't overly useful, nor does it really fit - but I had fun making it!

**RLTK_RS - Rust Library**

I've been hard at working taking the library code from RR, and turning it into a library: [https://github.com/thebracket/rltk_rs](https://github.com/thebracket/rltk_rs). I don't recommend *using* it until it hits version 0.2 - I'm still tweaking the API, so I'll potentially break anything you write at this point.

* It's focused on providing a simple API: minimal bootstrapping, and then clear, consise calls to render code.
* It's obsessively agnostic about how your code is arranged. Your map can implement the `Algorithm2D` trait, and teach it to translate x/y positions to whatever indexing scheme you are using behind the scenes. This then automatically provides distance and other geometry functions. Implement `BaseMap` (which requires that you provide an `is_opaque` function and a `get_available_exits` function - the latter listing directions you can travel from a tile) adapts path-finding (A-Star and Dijkstra) to your map format.
* It supports multiple render layers (with or without transparency), which can optionally use different font files. So if you want your GUI in a nicely readable 8x16 VGA font, and your game in a square 8x8 font - it's got your back. It should also work with tilesets, but I haven't tested that yet.
* There's a host of color support options. Colors are a basic `RGB` triple. I imported the X11 `rgb.txt` colorset (this is also used by the W3C for named colors in HTML/CSS), and setup helpers for them all. So if you want to use named colors, they are as simple as `RGB::named(rltk::YELLOW)`. It also knows `RGB::from_u8(255,255,0)`, `RGB::from_f32(1.0, 1.0, 0.0)`, and `RGB::from_hex("#FFFF00")`. There's various helpers such as `greyscale` (uses a fixed weighting to provide a quick to-grey function), `to_hsv` (turns it into a hue/saturation/value triplet), `lerp` (for stepping between colors), and basic math (add to/subtract from, multiply by, etc. between colors and colors and a float).
* I had some fun with the Dijkstra code, and have been trying to produce my fastest Dijkstra code to date. It now outperforms the code in *One Knight* by a large margin (although I'm busily porting it back to UE4/C++ since that's the largest slowdown remaining in the project). If you have a large number of starting points, it batches them based on the number of CPUs you have available and calculates Dijkstra maps for each batch - and then recombines. On a *release* build, it can handle thousands of target nodes without slowing down from 60 FPS now. Rust really did make the parallelization easy.

If you build it right now, it'll want you to have `cmake` and a full C++ toolchain available. That's because it goes out, downloads `glfw` (a C library) and builds it! I don't really like that at all - so I'm in the process of porting it over to `glutin` - a Rust-native OpenGL library. This is hard work, and I'm struggling to not change the API. It's in a branch (not sure if I've pushed that branch to github - been working on it locally), but it gets rid of the C toolchain requirement and is a little faster. In theory, it also supports web assembly (WASM) - untested, so far.

