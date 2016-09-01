use std;
use std::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use ansi_term::Colour::Blue;
use ansi_term::Style;
use ansi_term::ANSIStrings;

use posn::Posn;
use screen::move_cursor;
use tile::Tile;
use troll::Troll;
use direction::Direction;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Maze {
    pub map: Vec<Vec<Tile>>,
    pub trolls: HashMap<Posn, Troll>,
}

impl ::std::ops::Index<(usize, usize)> for Maze {
    type Output = Tile;
    fn index(&self, (row, col): (usize, usize)) -> &Tile {
        &self.map[row][col]
    }
}

impl<'c> ::std::ops::Index<&'c Posn> for Maze {
    type Output = Tile;
    fn index<'a, 'b>(&'a self, p: &'b Posn) -> &'a Tile {
        &self.map[p.row as usize][p.col as usize]
    }
}

impl<'c> ::std::ops::IndexMut<&'c Posn> for Maze {
    fn index_mut<'a, 'b>(&'a mut self, p: &'b Posn) -> &'a mut Tile {
        &mut self.map[p.row as usize][p.col as usize]
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut strings = vec![];
        let mut row = 0;
        for line in &self.map {
            let mut col = 0;
            for t in line.iter() {
                if let Some(troll) = self.trolls.get(&Posn {
                    row: row,
                    col: col,
                }) {
                    strings.push(troll.coloured());
                } else {
                    strings.push(t.coloured());
                }
                col += 1;
            }
            strings.push(Style::new().paint("\n"));
            row += 1
        }
        write!(f, "{}", ANSIStrings(&strings[..]))
    }
}


impl Maze {
    pub fn redraw_tile(&self, pos: &Posn) {
        assert!(self.in_bounds(pos));
        move_cursor(pos.row as usize, pos.col as usize);
        if self[pos] == Tile::Floor {
            if let Some(troll) = self.trolls.get(&pos) {
                print!("{}", troll.coloured());
                return;
            }

        }
        print!("{}", self[pos].coloured());
    }

    pub fn from_file(filename: &str) -> std::io::Result<Maze> {
        let f = try!(File::open(filename));
        let mut reader = BufReader::new(f);
        let mut maze = Maze {
            map: Vec::new(),
            trolls: HashMap::new(),
        };
        loop {
            let mut line = String::new();
            let size = try!(reader.read_line(&mut line));
            if size == 0 {
                break;
            }
            let maze_line = line[..size - 1]
                .chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    ' ' => Tile::Floor,
                    'X' => Tile::Exit,
                    _ => panic!("Bad maze character '{}'", c),
                })
                .collect();
            maze.map.push(maze_line);
        }
        Ok(maze)
    }

    pub fn add_troll(&mut self, pos: Posn, troll: Troll) {
        self.trolls.insert(pos, troll);
    }

    pub fn new(map: Vec<Vec<Tile>>) -> Maze {
        assert!(map.len() > 0);
        assert!(map[0].len() > 0);
        Maze {
            map: map,
            trolls: HashMap::new(),
        }
    }

    //                       row    col
    pub fn bounds(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn in_bounds(&self, pos: &Posn) -> bool {
        pos.row >= 0 && pos.row < self.map.len() as i32 && pos.col >= 0 &&
        pos.col < self.map[0].len() as i32
    }

    pub fn push(&mut self, pos: Posn, dir: Direction) {
        let next_tile_posn = pos + dir.numeric();
        if self.in_bounds(&next_tile_posn) {
            let next_tile = self[&next_tile_posn];
            if let Some(ref mut troll) = self.trolls.get_mut(&pos) {
                troll.alive = false;
            }
            if let Tile::Floor = next_tile {
                self[&next_tile_posn] = Tile::Wall;
                self[&pos] = Tile::Floor;
                self.redraw_tile(&pos);
                self.redraw_tile(&next_tile_posn);
            }
        }
    }

    pub fn random_floor_tile<R: Rng>(&self, rng: &mut R) -> Posn {
        let (max_row, max_col) = self.bounds();
        let mut counter = 0;
        loop {
            let row = Range::new(0, max_row as i32).ind_sample(rng);
            let col = Range::new(0, max_col as i32).ind_sample(rng);
            let pos = Posn {
                row: row,
                col: col,
            };
            if self[&pos] == Tile::Floor {
                return pos;
            }
            counter += 1;
            if counter > 10_000 {
                panic!("Can't find an empty floor tile");
            }
        }
    }
}

#[test]
fn test_maze_bounds() {
    use tile::Tile::*;
    let row = vec![Floor, Floor, Floor, Floor];
    let maze = Maze::new(vec![row.clone(), row.clone()]);
    assert_eq!(maze.in_bounds(&Posn { row: 0, col: 0 }), true);
    assert_eq!(maze.in_bounds(&Posn { row: 10, col: 10 }), false);
    assert_eq!(maze.in_bounds(&Posn { row: 1, col: 0 }), true);
    assert_eq!(maze.in_bounds(&Posn { row: 2, col: 0 }), false);
    assert_eq!(maze.in_bounds(&Posn { row: 1, col: 10 }), false);
    assert_eq!(maze.in_bounds(&Posn { row: 1, col: 3 }), true);
}
