extern crate ansi_term;
extern crate termios;
#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate rand;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::mem;

use rand::Rng;

mod direction;
mod maze;
mod player;
mod posn;
mod screen;
mod tile;
mod troll;

use direction::{Direction, North, South, East, West};
use maze::Maze;
use player::Player;
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
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            // This is a fairly simple format, though it's possible to do more complicated ones.
            // This closure can contain any code, as long as it produces a String message.
            format!("[{}][{}] {}",
                    time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                    level,
                    msg)
        }),
        output: vec![fern::OutputConfig::file("maze.log")],
        level: log::LogLevelFilter::Trace,
    };
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    let mut rng = rand::thread_rng();
    // let mut maze = Maze::from_file("maze.txt").unwrap();
    let mut maze = Maze::generate(10, 10, &mut rng);

    println!("q to Quit");
    println!("Maze bounds are {} by {}",
             maze.map.len(),
             maze.map[0].len());

    use termios::*;

    print!("\x1B[?1049h");
    print!("\x1B[1;1H");
    print!("\x1B[?25l");

    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut termios = Termios::from_fd(stdin.as_raw_fd()).unwrap();
    tcgetattr(stdin.as_raw_fd(), &mut termios).unwrap();
    let mut termios_old = termios.clone();
    termios.c_lflag = ISIG;
    termios.c_cc[VTIME] = 0;
    termios.c_cc[VMIN] = 1;
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios).unwrap();

    for _ in 0..3 {
        let tile = maze.random_floor_tile(&mut rng);
        maze.add_troll(tile, rng.gen())
    }

    print!("{}", maze);
    let mut player = Player {
        pos: maze.random_floor_tile(&mut rng),
        dir: Direction::North,
    };

    player.draw();
    ::std::io::stdout().flush().unwrap();

    info!("Starting game");

    'main_loop: loop {
        let mut input: [u8; 64] = [0; 64];
        let bytes = match stdin.read(&mut input) {
            Ok(n) => n,
            Err(_) => break 'main_loop,
        };
        let command = parse_keystroke(&input[..bytes]);
        let new_player = match command {
            None => continue,
            Some(Command::Quit) => break 'main_loop,
            Some(Command::Move(dir)) => {
                let mut new_player = player.clone();
                new_player.update(dir);
                new_player
            }
        };
        if maze.in_bounds(&new_player.pos) {
            match maze[&new_player.pos] {
                Tile::Floor => {
                    // We've moved the player.
                    maze.redraw_tile(&player.pos);
                    player = new_player;
                }
                Tile::Exit => {
                    println!("\nYou win!");
                    break 'main_loop;
                }
                Tile::Wall => {
                    maze.push(new_player.pos, new_player.dir);
                }
            }
        }
        let mut trolls = HashMap::new();
        mem::swap(&mut trolls, &mut maze.trolls);
        for (pos, mut troll) in trolls.into_iter() {
            maze.redraw_tile(&pos);
            let (pos, ate_player) = troll.update(pos, &mut maze, player.pos, &mut rng);
            maze.add_troll(pos, troll);
            maze.redraw_tile(&pos);
            if ate_player {
                break 'main_loop;
            }
        }

        player.draw();
        move_cursor(50, 0);
        print!("{},{}", player.pos.col, player.pos.row);
        ::std::io::stdout().flush().unwrap();
    }
    info!("Game over");
    // We always want to set the terminal to echo stuff, even if we didn't
    // start with it, as this fixes a broken terminal after a ctrl-c.
    termios_old.c_lflag = ICANON | ECHO | ECHOE | ECHOK | ECHONL;
    tcsetattr(stdin.as_raw_fd(), TCSAFLUSH, &termios_old).unwrap();
    print!("\x1B[?1049l");
    print!("\x1B[?25h");
}
