#[derive(Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }
}
