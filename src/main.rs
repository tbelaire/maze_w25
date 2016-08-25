use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

enum Tile {
    Floor,
    Wall,
    Exit
}

struct Maze {
    map: Vec<Vec<Tile>>
}

fn read_maze(filename: &str) -> std::io::Result<Maze> {
    let mut f = try!(File::open(filename));
    let mut reader = BufReader::new(f);
    let mut maze = Maze{ map: Vec::new() };
    loop {
        let mut line = String::new();
        let size = try!(reader.read_line(&mut line));
        if size == 0 {
            break;
        }
        println!("line: {}", line);
    }
    return Ok(maze);
}

fn main() {
    let maze = read_maze("maze.txt");
    match maze {
        Err(err) => println!("err is {:?}", err),
        Ok(maze) => println!("got maze"),
    }
}
