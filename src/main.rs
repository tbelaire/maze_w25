extern crate ansi_term;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

use ansi_term::Colour::{Red, Blue};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
    Exit
}

impl Tile {
    fn coloured(&self) -> ANSIString<'static> {
        match *self {
            Tile::Floor => Style::new().paint(" "),
            Tile::Wall => Red.paint("#"),
            Tile::Exit => Blue.paint("X"),
        }
    }
}


impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Tile::Floor => Red.paint(" "), // Less adjusting the colour
            Tile::Wall => Red.paint("#"),
            Tile::Exit => Blue.paint("X"),
        })
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Tile>>
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

fn read_maze(filename: &str) -> std::io::Result<Maze> {
    let f = try!(File::open(filename));
    let mut reader = BufReader::new(f);
    let mut maze = Maze{ map: Vec::new() };
    loop {
        let mut line = String::new();
        let size = try!(reader.read_line(&mut line));
        if size == 0 {
            break;
        }
        let maze_line = line[.. size -1 ].chars().map(|c| match c {
            '#' => Tile::Wall,
            ' ' => Tile::Floor,
            'X' => Tile::Exit,
            _   => panic!("Bad maze character '{}'", c)
        }).collect();
        maze.map.push(maze_line);
    }
    return Ok(maze);
}

fn main() {
    let maze = read_maze("maze.txt");
    match maze {
        Err(err) => println!("err is {:?}", err),
        Ok(maze) => println!("got maze\n{}", maze),
    }
}
