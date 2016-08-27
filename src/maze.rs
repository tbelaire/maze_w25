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


#[derive(Debug)]
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
    pub fn redraw_tile(&self, row: usize, col: usize) {
        move_cursor(row, col); // + C?
        print!("{}", self[(row - 1, col - 1)].coloured());
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
        return Ok(maze);
    }
}
