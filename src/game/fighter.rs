pub struct Fighter {
    max_hp : i32,
    hp: i32,
    defense: i32,
    power: i32
}

impl Fighter {
    pub fn new(max_hp: i32, defense: i32, power: i32) -> Fighter {
        return Fighter{
            max_hp: max_hp,
            hp: max_hp,
            defense: defense,
            power: power
        };
    }
}