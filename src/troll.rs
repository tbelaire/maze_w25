use std::fmt;
use std::borrow::Cow;

use rand::{Rand, Rng};

use ansi_term::Colour::{Red, Blue};
use ansi_term::ANSIString;

use direction::Direction;
use maze::Maze;
use posn::Posn;
use tile::Tile;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Wandering,
    Charging,
    Stunned(i32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Troll {
    pub dir: Direction,
    pub alive: bool,
    pub state: State,
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
            state: State::Wandering,
        }
    }

    pub fn coloured(&self) -> ANSIString {
        if self.alive {
            Blue.paint(self)
        } else {
            Red.paint(self)
        }
    }
    pub fn update<R: Rng>(&mut self,
                          mut pos: Posn,
                          maze: &mut Maze,
                          player_pos: Posn,
                          rng: &mut R)
                          -> (Posn, bool) {
        if !self.alive {
            return (pos, false);
        }
        self.state = match self.state {
            State::Wandering => {
                let dir: Direction = rng.gen();
                if self.dir == dir {
                    let new_pos = pos + dir.numeric();
                    if !maze.in_bounds(&new_pos) {
                        panic!("Troll wandered off the map");
                    }
                    if new_pos == player_pos {
                        return (new_pos, true);
                    }
                    if maze[&new_pos] == Tile::Floor {
                        pos = new_pos
                    }
                } else {
                    self.dir = dir;
                }
                let mut probe_pos = pos;
                let mut final_state = State::Wandering;
                loop {
                    probe_pos = probe_pos + dir.numeric();
                    if !maze.in_bounds(&probe_pos) {
                        break;
                    }
                    if probe_pos == player_pos {
                        final_state = State::Charging;
                        break;
                    }
                    if !(maze[&probe_pos] == Tile::Floor) {
                        break;
                    }
                }
                final_state
            }
            State::Charging => {
                let new_pos = pos + self.dir.numeric();
                if !maze.in_bounds(&new_pos) {
                    panic!("Troll charged off the map");
                }
                if new_pos == player_pos {
                    return (new_pos, true);
                }
                match maze[&new_pos] {
                    Tile::Floor => {
                        pos = new_pos;
                        State::Charging
                    }
                    Tile::Wall => {
                        maze.push(new_pos, self.dir);
                        State::Stunned(3)
                    }
                    Tile::Exit => State::Wandering,
                }
            }
            State::Stunned(mut counter) => {
                counter -= 1;
                if counter == 0 {
                    State::Wandering
                } else {
                    State::Stunned(counter)
                }
            }

        };
        (pos, false)
    }
}

impl Rand for Troll {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Troll::new(rng.gen())
    }
}
