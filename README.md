# Rusty Roguelike!

The fine folks over at [/r/roguelikedev](https://www.reddit.com/r/roguelikedev/new/) on Reddit are running a summer of learning to write roguelikes. I've written quite a few, but wanted to learn Rust. So this repo is about my learning Rust.

* There's an `rltk` folder containing a module, in which I'm using OpenGL to simulate a basic console renderer.
* The `main.rs` file is the boilerplate required to get this to run. Still working on improving that.
* `game` contains the actual game, broken into various files.

It's cheating to build both the library AND the game - but I don't know a better way to learn the language.

**Progress**

Hello world with an OpenGL console pretending to be CP437:

![Boring console image](/resources/RustHelloWorld2.JPG)

Moving @ around a random map:

![Animated GIF](/resources/RustyRoguelike.gif)

Generating a nicer map:

![Animated GIF](/resources/RustyRoguelike2.gif)

Field-of-view and visibility:

![Animated GIF](/resources/RustyRoguelike3.gif)

Mouse support and a variety of mobs:

![Animated GIF](/resources/RustyRoguelike4.gif)
