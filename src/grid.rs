use posn::Posn;

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(v: Vec<Vec<T>>) -> Grid<T> {
        Grid(v)
    }
}

impl<T> ::std::ops::Index<Posn> for Grid<T> {
    type Output = T;
    fn index(&self, p: Posn) -> &Self::Output {
        assert!(p.row >= 0);
        assert!(p.col >= 0);
        &self.0[p.row as usize][p.col as usize]
    }
}

impl<T> ::std::ops::IndexMut<Posn> for Grid<T> {
    fn index_mut(&mut self, p: Posn) -> &mut Self::Output {
        assert!(p.row >= 0);
        assert!(p.col >= 0);
        &mut self.0[p.row as usize][p.col as usize]
    }
}
