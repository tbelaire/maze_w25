
use std::fmt;

use ansi_term::Colour::{Red, Blue};
use ansi_term::Style;
use ansi_term::ANSIString;

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Floor,
    Wall,
    Exit,
}

impl Tile {
    pub fn coloured(&self) -> ANSIString<'static> {
        match *self {
            Tile::Floor => Style::new().paint(" "),
            Tile::Wall => Red.paint("#"),
            Tile::Exit => Blue.paint("X"),
        }
    }
}


impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Tile::Floor => Red.paint(" "), // Less adjusting the colour
                   Tile::Wall => Red.paint("#"),
                   Tile::Exit => Blue.paint("X"),
               })
    }
}
