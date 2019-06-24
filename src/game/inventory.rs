use super::Item;

pub struct Inventory {
    pub items : Vec<Item>,
    capacity: i32
}

impl Inventory {
    pub fn new(capacity : i32) -> Inventory {
        return Inventory{ items: Vec::new(), capacity: capacity };
    }

    pub fn add_item(&mut self, item : Item) -> Vec<String> {
        let mut result : Vec<String> = Vec::new();
        if self.items.len() as i32 > self.capacity {
            result.push("You cannot carry any more!".to_string());
        } else {
            result.push(format!("You pick up the {}", item.name));
            self.items.push(item);
        }
        return result;
    }
}