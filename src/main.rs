extern crate ansi_term;
extern crate termios;
#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

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


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Command {
    Move(Direction),
    Quit,
}

fn parse_keystroke(input: &[u8]) -> Option<Command> {
    use Command::*;
    match input {
        b"\x1B[A" => Some(Move(North)),
        b"\x1B[B" => Some(Move(South)),
        b"\x1B[C" => Some(Move(East)),
        b"\x1B[D" => Some(Move(West)),
        b"w" => Some(Move(North)),
        b"s" => Some(Move(South)),
        b"a" => Some(Move(West)),
        b"d" => Some(Move(East)),
        b"q" => Some(Quit),
        _ => None,
    }
}

fn main() {
    let mut maze = Maze::from_file("maze.txt").unwrap();

    let logger_config = fern::DispatchConfig {
    format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
        // This is a fairly simple format, though it's possible to do more complicated ones.
        // This closure can contain any code, as long as it produces a String message.
        format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
    }),
    output: vec![fern::OutputConfig::file("maze.log")],
    level: log::LogLevelFilter::Trace,
    };
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    println!("q to Quit");
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

    info!("Starting game");

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
                            trace!("Walking into wall");
                            let next_tile_posn = new_player.pos + dir.numeric();
                            if maze.in_bounds(&next_tile_posn) {
                                let next_tile = maze[&next_tile_posn];

                                trace!("Next tile ({:?}) is {:?}", next_tile_posn, next_tile);
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
    info!("Game over");
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios_old).unwrap();
    print!("\x1B[2J");
    print!("\x1B[1;1H");
    print!("\x1B[?25h");
}
