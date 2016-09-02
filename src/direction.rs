
use rand::{Rand, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
pub use self::Direction::*;

impl Direction {
    //                        row, col
    pub fn numeric(&self) -> (i32, i32) {
        match *self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
    pub fn unicode(&self) -> &'static str {
        match *self {
            North => "▲",
            South => "▼",
            East => "▶",
            West => "◀",
        }
    }
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        *rng.choose(&[North, South, East, West]).unwrap()
    }
}
