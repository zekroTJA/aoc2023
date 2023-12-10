use crate::pos::Pos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

impl From<Direction> for Pos {
    fn from(v: Direction) -> Self {
        match v {
            Direction::Up => Pos { x: 0, y: 1 },
            Direction::Down => Pos { x: 0, y: -1 },
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
        }
    }
}
