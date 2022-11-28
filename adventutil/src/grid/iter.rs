use super::{Cell, Coords, Direction, Grid};
use std::iter::FusedIterator;

pub struct IterCoords {
    height: usize,
    width: usize,
    y: usize,
    x: usize,
}

impl IterCoords {
    pub(super) fn new(height: usize, width: usize) -> IterCoords {
        IterCoords {
            height,
            width,
            y: 0,
            x: 0,
        }
    }
}

impl Iterator for IterCoords {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        if self.y >= self.height || self.x == self.width {
            return None;
        }
        let yx = Coords::new(self.y, self.x);
        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }
        Some(yx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.y >= self.height || self.x == self.width {
            return (0, Some(0));
        }
        let sz = self.width * (self.height - self.y) - self.x;
        (sz, Some(sz))
    }
}

impl FusedIterator for IterCoords {}

impl ExactSizeIterator for IterCoords {}

pub struct Enumerate<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Enumerate<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        Enumerate { grid, y: 0, x: 0 }
    }
}

impl<'a, T> Iterator for Enumerate<'a, T> {
    type Item = (Coords, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }
        let cell = self.grid.get((self.y, self.x)).unwrap();
        let r = (Coords::new(self.y, self.x), cell);
        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some(r)
    }
}

impl<'a, T> FusedIterator for Enumerate<'a, T> {}

pub struct IterCells<'a, T> {
    inner: IterCoords,
    grid: &'a Grid<T>,
}

impl<'a, T> IterCells<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        IterCells {
            inner: grid.iter_coords(),
            grid,
        }
    }
}

impl<'a, T> Iterator for IterCells<'a, T> {
    type Item = Cell<'a, T>;

    fn next(&mut self) -> Option<Cell<'a, T>> {
        let coords = self.inner.next()?;
        self.grid.get_cell(coords)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for IterCells<'a, T> {}

impl<'a, T> ExactSizeIterator for IterCells<'a, T> {}

pub struct Columns<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
}

impl<'a, T> Columns<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        Columns { grid, x: 0 }
    }
}

impl<'a, T> Iterator for Columns<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let col = self.grid.get_column(self.x);
        if col.is_some() {
            self.x += 1;
        }
        col
    }
}

impl<'a, T> FusedIterator for Columns<'a, T> {}

pub struct Cardinals(usize);

impl Cardinals {
    pub(super) fn new() -> Cardinals {
        Cardinals(0)
    }
}

impl Iterator for Cardinals {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        let d = match self.0 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => return None,
        };
        self.0 += 1;
        Some(d)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = 4usize.saturating_sub(self.0);
        (sz, Some(sz))
    }
}

impl FusedIterator for Cardinals {}

impl ExactSizeIterator for Cardinals {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enumerate() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.enumerate();
        assert_eq!(iter.next(), Some((Coords::new(0, 0), &1)));
        assert_eq!(iter.next(), Some((Coords::new(0, 1), &2)));
        assert_eq!(iter.next(), Some((Coords::new(0, 2), &3)));
        assert_eq!(iter.next(), Some((Coords::new(1, 0), &4)));
        assert_eq!(iter.next(), Some((Coords::new(1, 1), &5)));
        assert_eq!(iter.next(), Some((Coords::new(1, 2), &6)));
        assert_eq!(iter.next(), Some((Coords::new(2, 0), &7)));
        assert_eq!(iter.next(), Some((Coords::new(2, 1), &8)));
        assert_eq!(iter.next(), Some((Coords::new(2, 2), &9)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_coords_3x2() {
        let mut iter = IterCoords::new(3, 2);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(Coords::new(0, 0)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some(Coords::new(0, 1)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Coords::new(1, 0)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Coords::new(1, 1)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Coords::new(2, 0)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Coords::new(2, 1)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_coords_0x2() {
        let mut iter = IterCoords::new(0, 2);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_coords_3x0() {
        let mut iter = IterCoords::new(3, 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_coords_0x0() {
        let mut iter = IterCoords::new(0, 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_cells() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.iter_cells();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next().unwrap(), 6);
        assert_eq!(iter.next().unwrap(), 7);
        assert_eq!(iter.next().unwrap(), 8);
        assert_eq!(iter.next().unwrap(), 9);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_cardinals() {
        let mut iter = Direction::cardinals();
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Direction::North));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Direction::East));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Direction::South));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Direction::West));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
