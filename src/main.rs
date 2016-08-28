extern crate ansi_term;
extern crate termios;

use std::io::prelude::*;
use std::fs::File;
use std::os::unix::io::AsRawFd;

mod direction;
mod maze;
mod player;
mod posn;
mod screen;
mod tile;

use direction::{Direction, North, South, East, West};
use maze::Maze;
use player::Player;
use posn::Posn;
use screen::move_cursor;
use tile::Tile;


enum Command {
    Move(Direction),
    Quit,
}

fn parse_keystroke(input: &[u8]) -> Option<Command> {
    use Command::*;
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

fn main() {
    let mut maze = Maze::from_file("maze.txt").unwrap();

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
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios).unwrap();

    print!("{}", maze);
    let mut player = Player {
        pos: Posn { row: 4, col: 5 },
        dir: Direction::North,
    };

    player.draw();
    ::std::io::stdout().flush().unwrap();

    writeln!(&mut std::io::stderr(), "Starting game").unwrap();

    loop {
        let mut input: [u8; 64] = [0; 64];
        let bytes = match stdin.read(&mut input) {
            Ok(n) => n,
            Err(_) => break,
        };
        let command = parse_keystroke(&input[..bytes]);
        match command {
            None => {}
            Some(Command::Quit) => break,
            Some(Command::Move(dir)) => {
                let mut new_player = player.clone();
                new_player.update(dir);
                if maze.in_bounds(&new_player.pos) {
                    match maze[&new_player.pos] {
                        Tile::Floor => {
                            // We've moved the player.
                            maze.redraw_tile(&player.pos);
                            player = new_player;
                        }
                        Tile::Exit => {
                            println!("\nYou win!");
                            break;
                        }
                        Tile::Wall => {
                            writeln!(&mut std::io::stderr(), "Walking into wall").unwrap();
                            let next_tile_posn = new_player.pos + dir.numeric();
                            if maze.in_bounds(&next_tile_posn) {
                                let next_tile = maze[&next_tile_posn];
                                writeln!(&mut std::io::stderr(), "Next tile ({:?}) is {:?}",
                                next_tile_posn, next_tile).unwrap();
                                match next_tile {
                                    Tile::Floor => {
                                        maze[&next_tile_posn] = Tile::Wall;
                                        maze[&new_player.pos] = Tile::Floor;
                                        maze.redraw_tile(&new_player.pos);
                                        maze.redraw_tile(&next_tile_posn);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        player.draw();
        move_cursor(50, 0);
        print!("{},{}", player.pos.col, player.pos.row);
        ::std::io::stdout().flush().unwrap();
    }
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios_old).unwrap();
    print!("\x1B[2J");
    print!("\x1B[1;1H");
    print!("\x1B[?25h");
}
