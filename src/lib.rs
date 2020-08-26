pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Tile {
    pub value: u16,
}

pub struct Game {
    pub score: u32,
    pub tiles: Vec<Vec<Tile>>,
}
