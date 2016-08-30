use std::fmt;
use std::borrow::Cow;
use direction::Direction;

use ansi_term::Colour::{Red, Blue};
use ansi_term::Style;
use ansi_term::ANSIString;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Troll {
    pub dir: Direction,
    pub alive: bool,
}

impl fmt::Display for Troll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dir.unicode())
    }
}

impl<'a> Into<Cow<'static, str>> for &'a Troll {
    fn into(self) -> Cow<'static, str> {
        Cow::Borrowed(self.dir.unicode())
    }
}

impl Troll {
    pub fn new(dir: Direction) -> Troll {
        Troll {
            dir: dir,
            alive: true,
        }
    }

    pub fn coloured(&self) -> ANSIString {
        if self.alive {
            Blue.paint(self)
        } else {
            Red.paint(self)
        }
    }
}
