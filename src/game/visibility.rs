use crate::rltk;
use rltk::field_of_view;

use super::Map;
use super::Player;
use super::Mob;

pub trait Visibility {
    fn plot_visibility(&mut self, map : &Map);
}

impl Visibility for Player {
    fn plot_visibility(&mut self, map: &Map) {
        self.visible_tiles = field_of_view(&self.position, 6, map); 
    }
}

impl Visibility for Mob {
    fn plot_visibility(&mut self, map: &Map) {
        self.visible_tiles = field_of_view(&self.position, 6, map); 
    }
}