use super::{Cell, Coords, Direction, Grid, GridBounds};
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
    inner: IterCoords,
    grid: &'a Grid<T>,
}

impl<'a, T> Enumerate<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        Enumerate {
            inner: grid.iter_coords(),
            grid,
        }
    }
}

impl<'a, T> Iterator for Enumerate<'a, T> {
    type Item = (Coords, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let coords = self.inner.next()?;
        let value = self.grid.get(coords).unwrap();
        Some((coords, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for Enumerate<'a, T> {}

impl<'a, T> ExactSizeIterator for Enumerate<'a, T> {}

pub struct IntoIter<T> {
    coords_iter: IterCoords,
    rows_iter: std::vec::IntoIter<Vec<T>>,
    row: Option<std::vec::IntoIter<T>>,
}

impl<T> IntoIter<T> {
    pub(super) fn new(grid: Grid<T>) -> Self {
        let coords_iter = grid.iter_coords();
        IntoIter {
            coords_iter,
            rows_iter: grid.data.into_iter(),
            row: None,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = (Coords, T);

    fn next(&mut self) -> Option<Self::Item> {
        let coords = self.coords_iter.next()?;
        loop {
            if self.row.is_none() {
                self.row = Some(self.rows_iter.next()?.into_iter());
            }
            match self.row.as_mut().unwrap().next() {
                Some(value) => return Some((coords, value)),
                None => self.row = None,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.coords_iter.size_hint()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> ExactSizeIterator for IntoIter<T> {}

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.grid.width().saturating_sub(self.x);
        (sz, Some(sz))
    }
}

impl<'a, T> FusedIterator for Columns<'a, T> {}

impl<'a, T> ExactSizeIterator for Columns<'a, T> {}

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

pub struct AdjacentDirs(usize);

impl AdjacentDirs {
    pub(super) fn new() -> AdjacentDirs {
        AdjacentDirs(0)
    }
}

impl Iterator for AdjacentDirs {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        let d = match self.0 {
            0 => Direction::NorthWest,
            1 => Direction::North,
            2 => Direction::NorthEast,
            3 => Direction::West,
            4 => Direction::East,
            5 => Direction::SouthWest,
            6 => Direction::South,
            7 => Direction::SouthEast,
            _ => return None,
        };
        self.0 += 1;
        Some(d)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = 8usize.saturating_sub(self.0);
        (sz, Some(sz))
    }
}

impl FusedIterator for AdjacentDirs {}

impl ExactSizeIterator for AdjacentDirs {}

pub struct AdjacentCells<'a, T> {
    grid: &'a Grid<T>,
    center: Coords,
    bounds: GridBounds,
    inner: AdjacentDirs,
}

impl<'a, T> AdjacentCells<'a, T> {
    pub(super) fn new(cell: &Cell<'a, T>) -> Self {
        let grid = cell.grid();
        AdjacentCells {
            grid,
            center: cell.coords(),
            bounds: grid.bounds(),
            inner: Direction::adjacent(),
        }
    }
}

impl<'a, T> Iterator for AdjacentCells<'a, T> {
    type Item = Cell<'a, T>;

    fn next(&mut self) -> Option<Cell<'a, T>> {
        for d in self.inner.by_ref() {
            if let Some(c) = self.bounds.move_in(self.center, d) {
                return self.grid.get_cell(c);
            }
        }
        None
    }
}

impl<'a, T> FusedIterator for AdjacentCells<'a, T> {}

pub struct AdjacentWrapCells<'a, T> {
    grid: &'a Grid<T>,
    center: Coords,
    bounds: GridBounds,
    inner: AdjacentDirs,
}

impl<'a, T> AdjacentWrapCells<'a, T> {
    pub(super) fn new(cell: &Cell<'a, T>) -> Self {
        let grid = cell.grid();
        AdjacentWrapCells {
            grid,
            center: cell.coords(),
            bounds: grid.bounds(),
            inner: Direction::adjacent(),
        }
    }
}

impl<'a, T> Iterator for AdjacentWrapCells<'a, T> {
    type Item = Cell<'a, T>;

    fn next(&mut self) -> Option<Cell<'a, T>> {
        let d = self.inner.next()?;
        let c = self.bounds.move_in_wrap(self.center, d);
        self.grid.get_cell(c)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for AdjacentWrapCells<'a, T> {}

impl<'a, T> ExactSizeIterator for AdjacentWrapCells<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.enumerate();
        assert_eq!(iter.size_hint(), (9, Some(9)));
        assert_eq!(iter.next(), Some((Coords::new(0, 0), &1)));
        assert_eq!(iter.size_hint(), (8, Some(8)));
        assert_eq!(iter.next(), Some((Coords::new(0, 1), &2)));
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next(), Some((Coords::new(0, 2), &3)));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((Coords::new(1, 0), &4)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((Coords::new(1, 1), &5)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((Coords::new(1, 2), &6)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((Coords::new(2, 0), &7)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((Coords::new(2, 1), &8)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((Coords::new(2, 2), &9)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.into_iter();
        assert_eq!(iter.size_hint(), (9, Some(9)));
        assert_eq!(iter.next(), Some((Coords::new(0, 0), 1)));
        assert_eq!(iter.size_hint(), (8, Some(8)));
        assert_eq!(iter.next(), Some((Coords::new(0, 1), 2)));
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next(), Some((Coords::new(0, 2), 3)));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((Coords::new(1, 0), 4)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((Coords::new(1, 1), 5)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((Coords::new(1, 2), 6)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((Coords::new(2, 0), 7)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((Coords::new(2, 1), 8)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((Coords::new(2, 2), 9)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_columns() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.columns();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(vec![&1, &4, &7]));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(vec![&2, &5, &8]));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(vec![&3, &6, &9]));
        assert_eq!(iter.size_hint(), (0, Some(0)));
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

    #[test]
    fn test_adjacent_dirs() {
        let mut iter = Direction::adjacent();
        assert_eq!(iter.size_hint(), (8, Some(8)));
        assert_eq!(iter.next(), Some(Direction::NorthWest));
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next(), Some(Direction::North));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(Direction::NorthEast));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some(Direction::West));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Direction::East));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Direction::SouthWest));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Direction::South));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Direction::SouthEast));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
