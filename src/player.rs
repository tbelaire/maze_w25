use std::fmt;

use ansi_term::Colour::Green;

use direction::Direction;
use posn::Posn;

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: Posn,
    pub dir: Direction,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\x1B7\x1B[{row};{col}f{character}",
               row = self.pos.row,
               col = self.pos.col,
               character = Green.paint("&"))
    }
}
