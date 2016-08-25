extern crate ansi_term;
extern crate termios;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

use ansi_term::Colour::{Red, Blue, Green};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};

use std::os::unix::io::{IntoRawFd, AsRawFd, RawFd};

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

enum Direction {
    North,
    East,
    South,
    West,
}

struct Player {
    row: usize,
    col: usize,
    dir: Direction,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B7\x1B[{row};{col}f{character}",
               row=self.row, col=self.col,
               character=Green.paint("&"))
    }
}

fn main() {
    let maze = read_maze("maze.txt").unwrap();

    println!("Maze bounds are {} by {}",
             maze.map.len(), maze.map[0].len());

    use termios::*;

    print!("\x1B[2J");
    print!("\x1B[1;1H");
    print!("\x1B[?25l");

    let termios_old: Termios;
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut termios = Termios::from_fd(stdin.as_raw_fd()).unwrap();
    tcgetattr(stdin.as_raw_fd(), &mut termios).unwrap();
    termios_old = termios.clone();
    termios.c_lflag = ISIG;
    termios.c_cc[VTIME] = 0;
    termios.c_cc[VMIN] = 1;
    // cfmakeraw(&mut termios);
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios).unwrap();

    print!("{}", maze);
    let mut player = Player{ row:4, col: 4, dir:Direction::North };

    print!("{}", player);
    ::std::io::stdout().flush().unwrap();

    loop{

        let mut input : [u8; 64] = [0; 64];
        let mut bytes = match stdin.read(&mut input) {
            Ok(n) => n,
            Err(_) => break,
        };
        if bytes == 3 {
            if input[0] == 0x1B && input[1] == b'[' {
                match input[2] {
                    b'A' =>  player.row -= 1,
                    b'B' =>  player.row += 1,
                    b'C' =>  player.col += 1,
                    b'D' =>  player.col -= 1,
                    _ => panic!("unknown escape sequence"),
                }
            }
        } else {
            break;
        }

        print!("{}", player);
        ::std::io::stdout().flush().unwrap();
    }
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios_old).unwrap();
    print!("\x1B[2J");
    print!("\x1B[1;1H");
    print!("\x1B[?25h");
}
