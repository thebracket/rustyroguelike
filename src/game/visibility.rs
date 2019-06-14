use crate::rltk;
use rltk::field_of_view;
use rltk::Point;

use super::Map;
use super::Player;
use super::Mob;

pub trait Visibility {
    fn plot_visibility(&mut self, map : &Map);
}

impl Visibility for Player {
    fn plot_visibility(&mut self, map: &Map) {
        let fov_check = |pt : &Point|->bool { map.is_transparent(pt.x, pt.y) };
        self.visible_tiles = field_of_view(&self.position, 6, &fov_check); 
    }
}

impl Visibility for Mob {
    fn plot_visibility(&mut self, map: &Map) {
        let fov_check = |pt : &Point|->bool { map.is_transparent(pt.x, pt.y) };
        self.visible_tiles = field_of_view(&self.position, 6, &fov_check); 
    }
}