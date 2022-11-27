// This uses a (y, x) coordinate system in which the origin is in the top-left
// (north-west) corner.
use std::cmp::Ordering;
use std::fmt;
use std::iter::FusedIterator;
use std::ops::{Deref, Range};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    // Invariants:
    // - `data` is nonempty.
    // - Every row in `data` is nonempty.
    // - Every row in `data` has the same length.
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn bounds(&self) -> GridBounds {
        GridBounds::new(self.height(), self.width())
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_wrap(&self, y: isize, x: isize) -> &T {
        self.get(iurem(y, self.height()), iurem(x, self.width()))
            .unwrap()
    }

    pub fn get_cell(&self, y: usize, x: usize) -> Option<Cell<'_, T>> {
        if (0..self.height()).contains(&y) && (0..self.width()).contains(&x) {
            Some(Cell::new(self, y, x))
        } else {
            None
        }
    }

    pub fn map<U, F>(self, mut f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        Grid {
            data: self
                .data
                .into_iter()
                .map(|row| row.into_iter().map(&mut f).collect())
                .collect(),
        }
    }

    pub fn try_map<U, E, F>(self, mut f: F) -> Result<Grid<U>, E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let mut data = Vec::with_capacity(self.data.len());
        for row in self.data {
            let mut new_row = Vec::with_capacity(row.len());
            for value in row {
                new_row.push(f(value)?);
            }
            data.push(new_row);
        }
        Ok(Grid { data })
    }

    pub fn map_cell<U, F>(&self, mut f: F) -> Grid<U>
    where
        F: FnMut(Cell<'_, T>) -> U,
    {
        let mut data = Vec::with_capacity(self.height());
        for y in 0..self.height() {
            let mut new_row = Vec::with_capacity(self.width());
            for x in 0..self.width() {
                let cell = Cell::new(self, y, x);
                new_row.push(f(cell));
            }
            data.push(new_row);
        }
        Grid { data }
    }

    pub fn enumerate(&self) -> Enumerate<'_, T> {
        Enumerate::new(self)
    }

    pub fn columns(&self) -> Columns<'_, T> {
        Columns::new(self)
    }

    pub fn get_column(&self, x: usize) -> Option<Vec<&T>> {
        if x < self.width() {
            Some(
                (0..self.height())
                    .map(|y| self.get(y, x).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn filter_rows<P>(self, predicate: P) -> Option<Grid<T>>
    where
        P: FnMut(&Vec<T>) -> bool,
    {
        Grid::try_from(self.data.into_iter().filter(predicate).collect::<Vec<_>>()).ok()
    }

    pub fn into_rows(self) -> impl Iterator<Item = Vec<T>> {
        self.data.into_iter()
    }

    pub fn iter_coords(&self) -> IterCoords {
        IterCoords::new(self.height(), self.width())
    }

    pub fn iter_cells(&self) -> IterCells<'_, T> {
        IterCells::new(self)
    }
}

impl<T: FromStr> Grid<T> {
    pub fn parse_words(s: &str) -> Result<Grid<T>, ParseGridError<<T as FromStr>::Err>> {
        Grid::try_from(
            s.lines()
                .map(|l| l.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )?
        .try_map(|s| s.parse::<T>())
        .map_err(ParseGridError::Parse)
    }
}

fn iurem(x: isize, y: usize) -> usize {
    let r = match y.try_into() {
        Ok(y) => x.rem_euclid(y),
        Err(_) => panic!("Cannot take remainder with mixed isize and usize: modulus out of range"),
    };
    r.try_into().unwrap()
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridFromError;

    fn try_from(data: Vec<Vec<T>>) -> Result<Grid<T>, GridFromError> {
        let width = match data.get(0) {
            Some(row) => row.len(),
            None => return Err(GridFromError::Empty),
        };
        for row in &data[1..] {
            if row.len() != width {
                return Err(GridFromError::Ragged);
            }
        }
        if width == 0 {
            return Err(GridFromError::Empty);
        }
        Ok(Grid { data })
    }
}

impl<T: FromStr> FromStr for Grid<T> {
    type Err = ParseGridError<<T as FromStr>::Err>;

    fn from_str(s: &str) -> Result<Grid<T>, Self::Err> {
        Grid::try_from(
            s.lines()
                .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )?
        .try_map(|s| s.parse::<T>())
        .map_err(ParseGridError::Parse)
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GridFromError {
    #[error("Input grid is empty")]
    Empty,
    #[error("Input grid is ragged/uneven")]
    Ragged,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ParseGridError<E> {
    #[error("Input is not a grid: {0}")]
    From(#[from] GridFromError),
    #[error("Error parsing cells: {0}")]
    Parse(#[source] E),
}

pub struct Enumerate<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Enumerate<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Enumerate { grid, y: 0, x: 0 }
    }
}

impl<'a, T> Iterator for Enumerate<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }
        let cell = self.grid.get(self.y, self.x).unwrap();
        let r = ((self.y, self.x), cell);
        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some(r)
    }
}

impl<'a, T> FusedIterator for Enumerate<'a, T> {}

pub struct Columns<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
}

impl<'a, T> Columns<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Cell<'a, T> {
    fn new(grid: &'a Grid<T>, y: usize, x: usize) -> Self {
        Cell { grid, y, x }
    }

    pub fn get(&self) -> &T {
        self.grid.get(self.y, self.x).unwrap()
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    pub fn neighbor(&self, d: Direction) -> Option<Cell<'_, T>> {
        let (y, x) = self.grid.bounds().move_in(self.coords(), d)?;
        self.grid.get_cell(y, x)
    }

    pub fn neighbor_wrap(&self, d: Direction) -> Cell<'_, T> {
        let (y, x) = self.grid.bounds().move_in_wrap(self.coords(), d);
        self.grid.get_cell(y, x).unwrap()
    }

    pub fn north(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::North)
    }

    pub fn north_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::North)
    }

    pub fn south(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::South)
    }

    pub fn south_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::South)
    }

    pub fn east(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::East)
    }

    pub fn east_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::East)
    }

    pub fn west(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::West)
    }

    pub fn west_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::West)
    }

    pub fn north_east(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::NorthEast)
    }

    pub fn north_east_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::NorthEast)
    }

    pub fn north_west(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::NorthWest)
    }

    pub fn north_west_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::NorthWest)
    }

    pub fn south_east(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::SouthEast)
    }

    pub fn south_east_wrap(&self) -> Cell<'_, T> {
        self.neighbor_wrap(Direction::SouthEast)
    }

    pub fn south_west(&self) -> Option<Cell<'_, T>> {
        self.neighbor(Direction::SouthWest)
    }

    pub fn south_west_wrap(&self) -> Cell<'_, T> {
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

pub struct IterCoords {
    height: usize,
    width: usize,
    y: usize,
    x: usize,
}

impl IterCoords {
    fn new(height: usize, width: usize) -> IterCoords {
        IterCoords {
            height,
            width,
            y: 0,
            x: 0,
        }
    }
}

impl Iterator for IterCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.y >= self.height || self.x == self.width {
            return None;
        }
        let yx = (self.y, self.x);
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

pub struct IterCells<'a, T> {
    inner: IterCoords,
    grid: &'a Grid<T>,
}

impl<'a, T> IterCells<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        IterCells {
            inner: grid.iter_coords(),
            grid,
        }
    }
}

impl<'a, T> Iterator for IterCells<'a, T> {
    type Item = Cell<'a, T>;

    fn next(&mut self) -> Option<Cell<'a, T>> {
        let (y, x) = self.inner.next()?;
        self.grid.get_cell(y, x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for IterCells<'a, T> {}

impl<'a, T> ExactSizeIterator for IterCells<'a, T> {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    Here,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    pub fn cardinals() -> Cardinals {
        Cardinals::new()
    }

    pub fn decompose(&self) -> (Ordering, Ordering) {
        use Direction::*;
        use Ordering::*;
        match self {
            NorthWest => (Less, Less),
            North => (Less, Equal),
            NorthEast => (Less, Greater),
            West => (Equal, Less),
            Here => (Equal, Equal),
            East => (Equal, Greater),
            SouthWest => (Greater, Less),
            South => (Greater, Equal),
            SouthEast => (Greater, Greater),
        }
    }
}

pub struct Cardinals(usize);

impl Cardinals {
    fn new() -> Cardinals {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridBounds {
    pub height: usize,
    pub width: usize,
}

impl GridBounds {
    pub fn new(height: usize, width: usize) -> GridBounds {
        GridBounds { height, width }
    }

    pub fn contains(&self, (y, x): (usize, usize)) -> bool {
        (0..self.height).contains(&y) && (0..self.width).contains(&x)
    }

    pub fn move_in(&self, (y, x): (usize, usize), d: Direction) -> Option<(usize, usize)> {
        let (ydiff, xdiff) = d.decompose();
        let y = move_in_range(y, 0..self.height, ydiff)?;
        let x = move_in_range(x, 0..self.width, xdiff)?;
        Some((y, x))
    }

    pub fn move_in_wrap(&self, (y, x): (usize, usize), d: Direction) -> (usize, usize) {
        let (ydiff, xdiff) = d.decompose();
        let y = move_in_range_wrap(y, 0..self.height, ydiff);
        let x = move_in_range_wrap(x, 0..self.width, xdiff);
        (y, x)
    }
}

impl IntoIterator for GridBounds {
    type Item = (usize, usize);
    type IntoIter = IterCoords;

    fn into_iter(self) -> IterCoords {
        IterCoords::new(self.height, self.width)
    }
}

fn move_in_range(x: usize, range: Range<usize>, delta: Ordering) -> Option<usize> {
    let x = match delta {
        Ordering::Less => x.checked_sub(1)?,
        Ordering::Equal => x,
        Ordering::Greater => x.checked_add(1)?,
    };
    range.contains(&x).then_some(x)
}

fn move_in_range_wrap(x: usize, range: Range<usize>, delta: Ordering) -> usize {
    if range.is_empty() {
        panic!("Empty range");
    }
    let x = match delta {
        Ordering::Less => x.checked_sub(1).unwrap_or(range.end - 1),
        Ordering::Equal => x,
        Ordering::Greater => x + 1,
    };
    if x < range.start {
        range.end - 1
    } else if x >= range.end {
        range.start
    } else {
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_grid_char() {
        let gr = "abc\ndef\nghi\n".parse::<Grid<char>>().unwrap();
        assert_eq!(
            gr,
            Grid {
                data: vec![
                    vec!['a', 'b', 'c'],
                    vec!['d', 'e', 'f'],
                    vec!['g', 'h', 'i']
                ]
            }
        );
    }

    #[test]
    fn test_parse_grid_i32() {
        let gr = "123\n456\n789\n".parse::<Grid<i32>>().unwrap();
        assert_eq!(
            gr,
            Grid {
                data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            }
        );
    }

    #[test]
    fn test_parse_grid_i32_words() {
        let gr = Grid::<i32>::parse_words(concat!(
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        ))
        .unwrap();
        assert_eq!(
            gr,
            Grid {
                data: vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ]
            }
        );
    }

    #[test]
    fn test_enumerate() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut iter = gr.enumerate();
        assert_eq!(iter.next(), Some(((0, 0), &1)));
        assert_eq!(iter.next(), Some(((0, 1), &2)));
        assert_eq!(iter.next(), Some(((0, 2), &3)));
        assert_eq!(iter.next(), Some(((1, 0), &4)));
        assert_eq!(iter.next(), Some(((1, 1), &5)));
        assert_eq!(iter.next(), Some(((1, 2), &6)));
        assert_eq!(iter.next(), Some(((2, 0), &7)));
        assert_eq!(iter.next(), Some(((2, 1), &8)));
        assert_eq!(iter.next(), Some(((2, 2), &9)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_cell_corner() {
        let gr = Grid {
            data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let cell = gr.get_cell(0, 2).unwrap();
        assert_eq!(cell.get(), &3);
        assert_eq!(cell.y(), 0);
        assert_eq!(cell.x(), 2);
        assert_eq!(cell.coords(), (0, 2));
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

    #[test]
    fn test_iter_coords_3x2() {
        let mut iter = IterCoords::new(3, 2);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((2, 1)));
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
