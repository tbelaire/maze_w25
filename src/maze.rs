use std;
use std::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use ansi_term::Style;
use ansi_term::ANSIStrings;

use posn::{Posn, Adjacencies};
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

    pub fn generate<R: Rng>(height: usize, width: usize, rng: &mut R) -> Maze {
        let mut map = Vec::new();
        let full_row: Vec<Tile> = vec![Tile::Wall; width * 2 + 1];
        for _ in 0..2 * height + 1 {
            map.push(full_row.clone());
        }

        fn hunt(map: &Vec<Vec<Tile>>,
                height: usize,
                width: usize,
                finished_row: usize)
                -> Option<Posn> {
            for row in finished_row..height {
                for col in 0..width {
                    if map[2 * row + 1][2 * col + 1] == Tile::Wall {
                        return Some(Posn {
                            row: row as i32,
                            col: col as i32,
                        });
                    }
                }
            }
            None
        }
        fn lookup_center(map: &Vec<Vec<Tile>>, p: Posn) -> Tile {
            map[(2 * p.row + 1) as usize][(2 * p.col + 1) as usize]
        }
        fn lookup_center_mut(map: &mut Vec<Vec<Tile>>, p: Posn) -> &mut Tile {
            &mut map[(2 * p.row + 1) as usize][(2 * p.col + 1) as usize]
        }

        let mut finished_row: usize = 0;
        // Initialize the exit.
        map[1][0] = Tile::Exit;
        map[1][1] = Tile::Floor;
        info!("Generating map");
        loop {
            let curr = hunt(&map, height, width, finished_row);
            if curr == None {
                break;
            }
            let mut curr = curr.unwrap();
            info!("Hunt started new section at {:?}", curr);
            finished_row = curr.row as usize;
            let ads: Vec<Posn> = Adjacencies::new(curr)
                .filter(|&p| {
                    p.inside(Posn { row: 0, col: 0 },
                             Posn {
                                 row: height as i32,
                                 col: width as i32,
                             }) && lookup_center(&map, p) == Tile::Floor
                })
                .collect();
            let starting_from = rng.choose(&ads).unwrap();
            let wall = Posn {
                row: ((2 * curr.row + 1 + 2 * starting_from.row + 1) / 2) as i32,
                col: ((2 * curr.col + 1 + 2 * starting_from.col + 1) / 2) as i32,
            };
            map[wall.row as usize][wall.col as usize] = Tile::Floor;

            loop {
                assert_eq!(lookup_center(&map, curr), Tile::Wall);
                *lookup_center_mut(&mut map, curr) = Tile::Floor;
                let ads: Vec<Posn> = Adjacencies::new(curr)
                    .filter(|&p| {
                        p.inside(Posn { row: 0, col: 0 },
                                 Posn {
                                     row: height as i32,
                                     col: width as i32,
                                 }) && lookup_center(&map, p) == Tile::Wall
                    })
                    .collect();
                if let Some(&next) = rng.choose(&ads) {
                    let wall = Posn {
                        row: ((2 * curr.row + 1 + 2 * next.row + 1) / 2) as i32,
                        col: ((2 * curr.col + 1 + 2 * next.col + 1) / 2) as i32,
                    };
                    map[wall.row as usize][wall.col as usize] = Tile::Floor;
                    curr = next;
                } else {
                    info!("Exhausted possibilities at {:?}", curr);
                    break;
                }

            }
        }


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
