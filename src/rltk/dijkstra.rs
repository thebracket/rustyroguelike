use super::Point;
use std::i32::MAX;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct DijkstraMap {
    pub map : Vec<i32>,
    size_x : i32,
    size_y : i32,
    max_depth : i32
}

#[allow(dead_code)]
impl DijkstraMap {
    pub fn new(size_x : i32, size_y: i32, starts: &Vec<Point>, is_blocked : &Fn(&Point)->bool, max_depth : i32) -> DijkstraMap {
        let mut result : Vec<i32> = Vec::new();
        for _i in 0 .. (size_x * size_y) { result.push(MAX) }
        let mut d = DijkstraMap{ map : result, size_x : size_x, size_y : size_y, max_depth : max_depth};
        d.build(starts, is_blocked);
        return d;
    }

    fn idx(&self, pt : &Point) -> usize {
        return ((pt.y * self.size_x) + pt.x) as usize;
    }

    fn add_if_open_and_passable(&self, x : i32, y : i32, is_blocked : &Fn(&Point)->bool, open_list : &mut Vec<(usize, i32)>, closed_list : &mut HashSet<usize>, depth : i32) {
        if depth+1 > self.max_depth { return; }
        if x < 0 || x >= self.size_x || y < 0 || y >= self.size_y { return; }
        let target = Point::new(x, y);
        let idx = self.idx(&target);
        if closed_list.contains(&idx) { return; }
        if is_blocked(&target) { return; }

        closed_list.insert(self.idx(&target));
        open_list.push((self.idx(&target), depth+1));
    }

    fn build(&mut self, starts: &Vec<Point>, is_blocked : &Fn(&Point)->bool) {
        let mut open_list : Vec<(usize, i32)> = Vec::new();
        let mut closed_list : HashSet<usize> = HashSet::new();

        for start in starts.iter() {
            open_list.clear();
            closed_list.clear();
            open_list.push((self.idx(start), 0));

            while !open_list.is_empty() {
                let current_tile = open_list[0];
                let tile_idx = current_tile.0;
                let depth = current_tile.1;
                open_list.remove(0);

                if self.map[tile_idx] > depth {
                    let x = tile_idx as i32 % self.size_x;
                    let y = tile_idx as i32 / self.size_x;
                    self.map[tile_idx] = depth;

                    self.add_if_open_and_passable(x-1, y-1, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x+1, y-1, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x-1, y+1, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x+1, y+1, is_blocked, &mut open_list, &mut closed_list, depth);

                    self.add_if_open_and_passable(x-1, y, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x+1, y, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x, y-1, is_blocked, &mut open_list, &mut closed_list, depth);
                    self.add_if_open_and_passable(x, y+1, is_blocked, &mut open_list, &mut closed_list, depth);
                }
            }
        }
    }

    fn consider_exit(&self, exits : &mut Vec<(Point, i32)>, x: i32, y: i32) {
        if x < 0 || x >= self.size_x || y < 0 || y > self.size_y { return; }
        let pt = Point::new(x,y);
        let id = self.idx(&pt);
        let dist = self.map[id];
        if dist > 0 && dist < MAX {
            exits.push((pt.clone(), dist));
        }
    }

    pub fn find_lowest_exit(&self, pos : Point) -> Option<Point> {
        let mut exits : Vec<(Point, i32)> = Vec::new();
        self.consider_exit(&mut exits, pos.x-1, pos.y-1);
        self.consider_exit(&mut exits, pos.x+1, pos.y-1);
        self.consider_exit(&mut exits, pos.x+1, pos.y+1);
        self.consider_exit(&mut exits, pos.x-1, pos.y+1);
        self.consider_exit(&mut exits, pos.x-1, pos.y);
        self.consider_exit(&mut exits, pos.x+1, pos.y);
        self.consider_exit(&mut exits, pos.x, pos.y-1);
        self.consider_exit(&mut exits, pos.x, pos.y+1);

        if exits.is_empty() { return None; }
        exits.sort_by(|a,b| a.1.cmp(&b.1));

        return Some(exits[0].0);
    }
}

