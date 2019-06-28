use crate::rltk;
use rltk::{Point, Color, Rltk};
use super::{ State };
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Particle {
    position : Point,
    lifetime_ms : f32,
    fg : Color,
    bg : Color,
    glyph : u8
}

impl Particle {
    pub fn new(position:Point, fg:Color, bg:Color, glyph: u8, lifetime_ms : f32) -> Particle {
        return Particle{ position:position, fg:fg, bg:bg, glyph:glyph, lifetime_ms : lifetime_ms};
    }

    pub fn render(&self, ctx : &mut Rltk) {
        ctx.con().set(self.position, self.fg, self.bg, self.glyph);
    }
}

pub fn age_particles(gs : &mut State, ctx : &mut Rltk) {
    for p in gs.vfx.iter_mut() {
        p.lifetime_ms -= ctx.frame_time_ms;
    }
    gs.vfx.retain(|a| a.lifetime_ms > 0.0);
}