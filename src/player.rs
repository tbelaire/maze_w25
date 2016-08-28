use std::fmt;
use std::io::prelude::*;

use ansi_term::Colour::Green;

use direction::Direction;
use posn::Posn;
use screen::move_cursor;

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: Posn,
    pub dir: Direction,
}

impl Player {
    pub fn draw(&self) {
        assert!(self.pos.row >= 0);
        assert!(self.pos.col >= 0);

        move_cursor(self.pos.row as usize, self.pos.col as usize);
        print!("{}", self);
    }
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{character}", character = Green.paint("&"))
    }
}
