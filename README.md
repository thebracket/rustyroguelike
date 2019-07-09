# Rusty Roguelike!

The fine folks over at [/r/roguelikedev](https://www.reddit.com/r/roguelikedev/new/) on Reddit are running a summer of learning to write roguelikes. I've written quite a few, but wanted to learn Rust. So this repo is about my learning Rust.

* There's an `rltk` folder containing a module, in which I'm using OpenGL to simulate a basic console renderer.
* The `main.rs` file is the boilerplate required to get this to run. Still working on improving that.
* `game` contains the actual game, broken into various files.

It's cheating to build both the library AND the game - but I don't know a better way to learn the language.

**Update**: Now that I've finished spinning it off into its own project, Rusty Roguelike uses `rltk_rs` for all the back-end features. You can see the library side of things [in the rltk_rs repo](https://github.com/thebracket/rltk_rs).

**Progress**

Hello world with an OpenGL console pretending to be CP437:

![Boring console image](/screenshots/RustHelloWorld2.JPG)

Moving @ around a random map:

![Animated GIF](/screenshots/RustyRoguelike.gif)

Generating a nicer map:

![Animated GIF](/screenshots/RustyRoguelike2.gif)

Field-of-view and visibility:

![Animated GIF](/screenshots/RustyRoguelike3.gif)

Mouse support and a variety of mobs:

![Animated GIF](/screenshots/RustyRoguelike4.gif)

Dijkstra-flow map based pathfinding (A* will follow) for basic mob AI:

![Animated GIF](/screenshots/RustyRoguelike5.gif)

A-Star based pathfinding, and the beginnings of a user interface, log and end-game:

![Animated GIF](/screenshots/RustyRoguelike6.gif)

Nicer tooltips:

![Animated GIF](/screenshots/RustyRoguelike7.gif)

Pick up and use healing potions:

![Animated GIF](/screenshots/RustyRoguelike8.gif)

Load and Save the game, and a main menu (tutorial 10):

![Animated GIF](/screenshots/RustyRoguelike9.gif)

Added in nicer wall graphics, using a bitmask to detect the correct tile

![Animated GIF](/screenshots/RustyRoguelike10.gif)

Added in some basic particle effects to make things prettier.

![Animated GIF](/screenshots/RustyRoguelike11.gif)
