
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Posn {
    pub row: i32,
    pub col: i32,
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
