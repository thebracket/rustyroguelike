use crate::rltk;
use crate ::rltk::Console;
use rltk::{Point, RGB, Rltk};
use super::{ State };
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Particle {
    position : Point,
    lifetime_ms : f32,
    fg : RGB,
    bg : RGB,
    glyph : u8
}

impl Particle {
    pub fn new(position:Point, fg:RGB, bg:RGB, glyph: u8, lifetime_ms : f32) -> Particle {
        return Particle{ position:position, fg:fg, bg:bg, glyph:glyph, lifetime_ms : lifetime_ms};
    }

    pub fn render(&self, ctx : &mut Rltk) {
        ctx.set(self.position.x, self.position.y, self.fg, self.bg, self.glyph);
    }
}

pub fn age_particles(gs : &mut State, ctx : &mut Rltk) {
    for p in gs.vfx.iter_mut() {
        p.lifetime_ms -= ctx.frame_time_ms;
    }
    gs.vfx.retain(|a| a.lifetime_ms > 0.0);
}