use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => String::from("up"),
            Direction::Down => String::from("down"),
            Direction::Right => String::from("right"),
            Direction::Left => String::from("left"),
        }
    }
}

impl Direction {
    pub fn is_opposite(&self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Right, Direction::Left)
                | (Direction::Left, Direction::Right)
        )
    }
}
