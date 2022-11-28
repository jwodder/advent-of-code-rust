use super::{Coords, Direction, Grid};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Cell<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>, y: usize, x: usize) -> Self {
        Cell { grid, y, x }
    }

    pub fn get(&self) -> &T {
        self.grid.get((self.y, self.x)).unwrap()
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn coords(&self) -> Coords {
        Coords::new(self.y, self.x)
    }

    pub fn neighbor(&self, d: Direction) -> Option<Cell<'a, T>> {
        let coords = self.grid.bounds().move_in(self.coords(), d)?;
        self.grid.get_cell(coords)
    }

    pub fn neighbor_wrap(&self, d: Direction) -> Cell<'a, T> {
        let coords = self.grid.bounds().move_in_wrap(self.coords(), d);
        self.grid.get_cell(coords).unwrap()
    }

    pub fn north(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::North)
    }

    pub fn north_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::North)
    }

    pub fn south(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::South)
    }

    pub fn south_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::South)
    }

    pub fn east(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::East)
    }

    pub fn east_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::East)
    }

    pub fn west(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::West)
    }

    pub fn west_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::West)
    }

    pub fn north_east(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::NorthEast)
    }

    pub fn north_east_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::NorthEast)
    }

    pub fn north_west(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::NorthWest)
    }

    pub fn north_west_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::NorthWest)
    }

    pub fn south_east(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::SouthEast)
    }

    pub fn south_east_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::SouthEast)
    }

    pub fn south_west(&self) -> Option<Cell<'a, T>> {
        self.neighbor(Direction::SouthWest)
    }

    pub fn south_west_wrap(&self) -> Cell<'a, T> {
        self.neighbor_wrap(Direction::SouthWest)
    }
}

impl<'a, T> Deref for Cell<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get()
    }
}

impl<'a, T: PartialEq> PartialEq<T> for Cell<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

//impl<'a, T: PartialEq> PartialEq<Cell<'a, T>> for T {
//    fn eq(&self, other: &Cell<'a, T>) -> bool {
//        self == other.get()
//    }
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_corner() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let cell = gr.get_cell((0, 2)).unwrap();
        assert_eq!(cell.get(), &3);
        assert_eq!(cell.y(), 0);
        assert_eq!(cell.x(), 2);
        assert_eq!(cell.coords(), Coords::new(0, 2));
        assert_eq!(cell.north(), None);
        assert_eq!(cell.north_wrap(), 9);
        assert_eq!(cell.north_east(), None);
        assert_eq!(cell.north_east_wrap(), 7);
        assert_eq!(cell.east(), None);
        assert_eq!(cell.east_wrap(), 1);
        assert_eq!(cell.south_east(), None);
        assert_eq!(cell.south_east_wrap(), 4);
        assert_eq!(cell.south().unwrap(), 6);
        assert_eq!(cell.south_wrap(), 6);
        assert_eq!(cell.south_west().unwrap(), 5);
        assert_eq!(cell.south_west_wrap(), 5);
        assert_eq!(cell.west().unwrap(), 2);
        assert_eq!(cell.west_wrap(), 2);
        assert_eq!(cell.north_west(), None);
        assert_eq!(cell.north_west_wrap(), 8);
    }
}
