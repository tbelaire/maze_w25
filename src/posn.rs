use direction::Direction;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Posn {
    pub row: i32,
    pub col: i32,
}

impl Posn {
    pub fn inside(&self, upper_left: Posn, lower_right: Posn) -> bool {
        self.row >= upper_left.row && self.row < lower_right.row && self.col >= upper_left.row &&
        self.col < lower_right.col
    }

    pub fn average(self, other: Posn) -> Posn {
        let sum = self + other;
        Posn {
            row: sum.row / 2,
            col: sum.col / 2,
        }
    }

    pub fn direction_to(self, other: Posn) -> Direction {
        let dx = self.col - other.col;
        let dy = self.row - other.row;

        use direction::Direction::*;
        if dx.abs() > dy.abs() {
            if dx >= 0 { West } else { East }
        } else {
            if dy >= 0 { North } else { South }
        }
    }
}

impl ::std::ops::Add<Posn> for Posn {
    type Output = Posn;
    fn add(self, rhs: Posn) -> Posn {
        Posn {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl ::std::ops::Add<(i32, i32)> for Posn {
    type Output = Posn;
    fn add(self, (row, col): (i32, i32)) -> Posn {
        Posn {
            row: self.row + row,
            col: self.col + col,
        }
    }
}

pub struct Adjacencies {
    pos: Posn,
    dir: Direction,
    done: bool,
}

impl Adjacencies {
    pub fn new(pos: Posn) -> Adjacencies {
        Adjacencies {
            pos: pos,
            dir: Direction::North,
            done: false,
        }
    }
}

impl Iterator for Adjacencies {
    type Item = Posn;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let next_pos = self.pos + self.dir.numeric();
        let dir = {
            match self.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => {
                    self.done = true;
                    Direction::North
                }
            }
        };
        self.dir = dir;
        Some(next_pos)
    }
}

#[test]
fn test_posn_add() {
    let a = Posn { row: 1, col: 1 };
    let b = Posn { row: -1, col: 0 };
    let c = a + b;
    assert_eq!(c, Posn { row: 0, col: 1 });
    let d = c + (1, -1);
    assert_eq!(d, Posn { row: 1, col: 0 });
    let e = c + a;
    assert_eq!(e, Posn { row: 1, col: 2 });
}

#[test]
fn test_iter_adjacencies() {
    let p = Posn { row: 1, col: 1 };
    let ads: Vec<Posn> = Adjacencies::new(p).collect();
    assert_eq!(ads,
               vec![
              p + Direction::North.numeric(),
              p + Direction::East.numeric(),
              p + Direction::South.numeric(),
              p + Direction::West.numeric(),
              ]);
    let ads: Vec<Posn> = Adjacencies::new(Posn { row: 0, col: 1 })
        .filter(|&p| p.row >= 0 && p.row < 2 && p.col >= 0 && p.col < 2)
        .collect();
    assert_eq!(ads,
               vec![
               Posn{ row: 1, col: 1},
               Posn{ row: 0, col: 0},
               ]);
    let ads: Vec<Posn> = Adjacencies::new(Posn { row: 0, col: 1 })
        .filter(|&p| p.inside(Posn { row: 0, col: 0 }, Posn { row: 2, col: 2 }))
        .collect();
    assert_eq!(ads,
               vec![
               Posn{ row: 1, col: 1},
               Posn{ row: 0, col: 0},
               ]);

}

#[test]
fn test_dir_to() {
    use direction::Direction::*;
    let p = Posn { row: 1, col: 1 };
    for dir in &[North, South, East, West] {
        assert_eq!(p.direction_to(p + dir.numeric()), *dir);
    }
    assert_eq!(p.direction_to(p), North);
}
