use std;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use ansi_term::Style;
use ansi_term::ANSIStrings;

use posn::Posn;
use screen::move_cursor;
use tile::Tile;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Maze {
    pub map: Vec<Vec<Tile>>,
}

impl ::std::ops::Index<(usize, usize)> for Maze {
    type Output = Tile;
    fn index<'a>(&'a self, (row, col): (usize, usize)) -> &'a Tile {
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
        for ref line in &self.map {
            for t in line.iter() {
                strings.push(t.coloured());
            }
            strings.push(Style::new().paint("\n"));
        }
        write!(f, "{}", ANSIStrings(&strings[..]))
    }
}


impl Maze {
    pub fn redraw_tile(&self, pos: &Posn) {
        assert!(self.in_bounds(pos));
        move_cursor(pos.row as usize, pos.col as usize);
        print!("{}", self[pos].coloured());
    }

    pub fn from_file(filename: &str) -> std::io::Result<Maze> {
        let f = try!(File::open(filename));
        let mut reader = BufReader::new(f);
        let mut maze = Maze { map: Vec::new() };
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

    pub fn new(map: Vec<Vec<Tile>>) -> Maze {
        assert!(map.len() > 0);
        assert!(map[0].len() > 0);
        Maze { map: map }
    }

    pub fn in_bounds(&self, pos: &Posn) -> bool {
        pos.row >= 0 && pos.row < self.map.len() as i32 && pos.col >= 0 &&
        pos.col < self.map[0].len() as i32
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
