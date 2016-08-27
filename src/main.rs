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

mod posn;
use posn::Posn;

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
    Exit,
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
        write!(f,
               "{}",
               match *self {
                   Tile::Floor => Red.paint(" "), // Less adjusting the colour
                   Tile::Wall => Red.paint("#"),
                   Tile::Exit => Blue.paint("X"),
               })
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Tile>>,
}

impl std::ops::Index<(usize, usize)> for Maze {
    type Output = Tile;
    fn index<'a>(&'a self, (row, col): (usize, usize)) -> &'a Tile {
        &self.map[row][col]
    }
}

impl Maze {
    fn redraw_tile(&self, row: usize, col: usize) {
        move_cursor(row, col); // + C?
        print!("{}", self[(row - 1, col - 1)].coloured());
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
    fn from_file(filename: &str) -> std::io::Result<Maze> {
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

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    //                    row, col
    fn numeric(&self) -> (i32, i32) {
        use Direction::*;
        match *self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
}

#[derive(Clone, Debug)]
struct Player {
    row: usize,
    col: usize,
    dir: Direction,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\x1B7\x1B[{row};{col}f{character}",
               row = self.row,
               col = self.col,
               character = Green.paint("&"))
    }
}

enum Command {
    Move(Direction),
    Quit,
}

fn parse_keystroke(input: &[u8]) -> Option<Command> {
    use Command::*;
    use Direction::*;
    if input.len() == 3 {
        if input[0] == 0x1B && input[1] == b'[' {
            match input[2] {
                b'A' => Some(Move(North)),
                b'B' => Some(Move(South)),
                b'C' => Some(Move(East)),
                b'D' => Some(Move(West)),
                _ => panic!("unknown escape sequence"),
            }
        } else {
            None
        }
    } else if input.len() == 1 {
        if input[0] == b'q' {
            Some(Command::Quit)
        } else {
            None
        }
    } else {
        None
    }
}

fn move_cursor(row: usize, col: usize) {
    print!("\x1B7\x1B[{row};{col}f", row = row, col = col);
}
fn main() {
    let maze = Maze::from_file("maze.txt").unwrap();

    println!("Maze bounds are {} by {}",
             maze.map.len(),
             maze.map[0].len());

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
    let mut player = Player {
        row: 4,
        col: 4,
        dir: Direction::North,
    };

    print!("{}", player);
    ::std::io::stdout().flush().unwrap();

    loop {

        let mut input: [u8; 64] = [0; 64];
        let mut bytes = match stdin.read(&mut input) {
            Ok(n) => n,
            Err(_) => break,
        };
        let command = parse_keystroke(&input[..bytes]);
        if let Some(command) = command {
            match command {
                Command::Quit => break,
                Command::Move(dir) => {
                    let old_player = player.clone();
                    use Direction::*;
                    match dir {
                        North => player.row -= 1,
                        South => player.row += 1,
                        East => player.col += 1,
                        West => player.col -= 1,
                    }
                    maze.redraw_tile(old_player.row, old_player.col);
                }
            }

            print!("{}", player);
            ::std::io::stdout().flush().unwrap();
        }
    }
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios_old).unwrap();
    print!("\x1B[2J");
    print!("\x1B[1;1H");
    print!("\x1B[?25h");
}
