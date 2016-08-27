use std::fmt;
use direction::Direction;

use ansi_term::Colour::Green;

#[derive(Clone, Debug)]
pub struct Player {
    pub row: usize,
    pub col: usize,
    pub dir: Direction,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\x1B7\x1B[{row};{col}f{character}",
               row = self.row,
               col = self.col,
               character = Green.paint("&"))
    }
}

