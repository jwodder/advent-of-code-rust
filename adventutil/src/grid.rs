// This uses a (y, x) coordinate system in which the origin is in the top-left
// (north-west) corner.
mod cell;
mod iter;
mod util;
pub use self::cell::*;
pub use self::iter::*;
use self::util::*;
use super::gridgeom::{Point, PointBounds};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Index, IndexMut, RangeBounds};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    // Invariants:
    // - `data` is nonempty.
    // - Every row in `data` is nonempty.
    // - Every row in `data` has the same length.
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn from_fn<F, C>(bounds: GridBounds, mut f: F) -> Grid<T>
    where
        F: FnMut(C) -> T,
        C: From<Coords>,
    {
        // TODO: Panic if `bounds` is empty
        Grid {
            data: (0..bounds.height)
                .map(|y| {
                    (0..bounds.width)
                        .map(|x| f(C::from(Coords::new(y, x))))
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }

    pub fn filled(bounds: GridBounds, value: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            data: vec![vec![value; bounds.width]; bounds.height],
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn bounds(&self) -> GridBounds {
        GridBounds::new(self.height(), self.width())
    }

    pub fn get<C: Into<(usize, usize)>>(&self, coords: C) -> Option<&T> {
        let (y, x) = coords.into();
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_wrap(&self, (y, x): (isize, isize)) -> &T {
        self.get((iurem(y, self.height()), iurem(x, self.width())))
            .unwrap()
    }

    pub fn get_cell<C: Into<(usize, usize)>>(&self, coords: C) -> Option<Cell<'_, T>> {
        let (y, x) = coords.into();
        self.bounds()
            .contains((y, x))
            .then(|| Cell::new(self, y, x))
    }

    pub fn get_mut<C: Into<(usize, usize)>>(&mut self, coords: C) -> Option<&mut T> {
        let (y, x) = coords.into();
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
    }

    // Panics on out-of-bounds
    pub fn set<C: Into<(usize, usize)>>(&mut self, coords: C, value: T) {
        let (y, x) = coords.into();
        self.data[y][x] = value;
    }

    // Panics on out-of-bounds
    pub fn row_slice<R: RangeBounds<usize>>(&self, range: R) -> Grid<T>
    where
        T: Clone,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        Grid {
            data: self.data[bounds].to_vec(),
        }
    }

    // Panics on out-of-bounds
    pub fn column_slice<R: RangeBounds<usize>>(&self, range: R) -> Grid<T>
    where
        T: Clone,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        Grid {
            data: self.data.iter().map(|row| row[bounds].to_vec()).collect(),
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

    pub fn map_cells<U, F>(&self, mut f: F) -> Grid<U>
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
        (x < self.width()).then(|| {
            (0..self.height())
                .map(|y| self.get((y, x)).unwrap())
                .collect()
        })
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

    pub fn into_values(self) -> impl Iterator<Item = T> {
        self.data.into_iter().flatten()
    }

    pub fn iter_coords(&self) -> IterCoords {
        IterCoords::new(self.height(), self.width())
    }

    pub fn iter_cells(&self) -> IterCells<'_, T> {
        IterCells::new(self)
    }

    pub fn embiggened(&self, padding: Padding, fill: T) -> Grid<T>
    where
        T: Clone,
    {
        let newbounds = GridBounds {
            height: self.height() + padding.top + padding.bottom,
            width: self.width() + padding.left + padding.right,
        };
        Grid::from_fn(newbounds, |(y, x)| {
            y.checked_sub(padding.top)
                .zip(x.checked_sub(padding.left))
                .filter(|&c| self.bounds().contains(c))
                .map_or_else(|| fill.clone(), |(oldy, oldx)| self[(oldy, oldx)].clone())
        })
    }
}

impl Grid<bool> {
    pub fn draw(&self) -> Draw<'_> {
        Draw(self)
    }

    pub fn from_drawing(s: &str) -> Result<Grid<bool>, GridFromError> {
        Grid::try_from(
            s.lines()
                .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    #[cfg(feature = "ocr")]
    pub fn ocr(self) -> anyhow::Result<String> {
        super::ocr::ocr(self)
    }

    pub fn from_points<I: IntoIterator<Item = Point>>(iter: I, downwards: bool) -> Grid<bool> {
        let mut iter = iter.into_iter();
        let p1 = iter.next().expect("Empty collection of points");
        let mut bounds = PointBounds::for_point(p1);
        let mut points = HashSet::from([p1]);
        for p in iter {
            bounds = bounds.with_point(p);
            points.insert(p);
        }
        let grbounds = GridBounds::new(
            usize::try_from(bounds.height()).unwrap(),
            usize::try_from(bounds.width()).unwrap(),
        );
        Grid::from_fn(grbounds, move |c| {
            points.contains(&bounds.at_coords(c, downwards))
        })
    }

    pub fn into_true_coords(self) -> IntoTrueCoords {
        IntoTrueCoords::new(self)
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Coords, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self)
    }
}

impl<C: Into<(usize, usize)>, T> Index<C> for Grid<T> {
    type Output = T;

    fn index(&self, index: C) -> &T {
        self.get(index).unwrap()
    }
}

impl<C: Into<(usize, usize)>, T> IndexMut<C> for Grid<T> {
    fn index_mut(&mut self, index: C) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: FromStr> Grid<T> {
    pub fn parse_words(s: &str) -> Result<Grid<T>, ParseGridError<<T as FromStr>::Err>> {
        Grid::try_from(
            s.lines()
                .map(|l| l.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )?
        .try_map(str::parse::<T>)
        .map_err(ParseGridError::Parse)
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for row in &self.data {
            if !std::mem::replace(&mut first, false) {
                writeln!(f)?;
            }
            for cell in row {
                write!(f, "{cell}")?;
            }
        }
        Ok(())
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridFromError;

    fn try_from(data: Vec<Vec<T>>) -> Result<Grid<T>, GridFromError> {
        let width = match data.first() {
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
    #[error("input grid is empty")]
    Empty,
    #[error("input grid is ragged/uneven")]
    Ragged,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ParseGridError<E> {
    #[error("input is not a grid: {0}")]
    From(#[from] GridFromError),
    #[error("error parsing cells: {0}")]
    Parse(#[source] E),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Draw<'a>(&'a Grid<bool>);

impl fmt::Display for Draw<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for row in &self.0.data {
            if !std::mem::replace(&mut first, false) {
                writeln!(f)?;
            }
            for &cell in row {
                write!(f, "{}", if cell { '#' } else { '.' })?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub y: usize,
    pub x: usize,
}

impl Coords {
    pub fn new(y: usize, x: usize) -> Coords {
        Coords { y, x }
    }
}

impl From<(usize, usize)> for Coords {
    fn from((y, x): (usize, usize)) -> Coords {
        Coords::new(y, x)
    }
}

impl From<Coords> for (usize, usize) {
    fn from(coords: Coords) -> (usize, usize) {
        (coords.y, coords.x)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridBounds {
    pub height: usize,
    pub width: usize,
}

impl GridBounds {
    pub fn new(height: usize, width: usize) -> GridBounds {
        GridBounds { height, width }
    }

    pub fn contains<C: Into<(usize, usize)>>(&self, coords: C) -> bool {
        let (y, x) = coords.into();
        (0..self.height).contains(&y) && (0..self.width).contains(&x)
    }

    pub fn wrap(&self, (y, x): (isize, isize)) -> Coords {
        Coords::new(iurem(y, self.height), iurem(x, self.width))
    }

    pub fn move_in<C: Into<(usize, usize)>>(&self, coords: C, d: Direction) -> Option<Coords> {
        let (y, x) = coords.into();
        let (ydiff, xdiff) = d.decompose();
        let y = move_in_range(y, 0..self.height, ydiff)?;
        let x = move_in_range(x, 0..self.width, xdiff)?;
        Some(Coords::new(y, x))
    }

    pub fn move_in_wrap<C: Into<(usize, usize)>>(&self, coords: C, d: Direction) -> Coords {
        let (y, x) = coords.into();
        let (ydiff, xdiff) = d.decompose();
        let y = move_in_range_wrap(y, 0..self.height, ydiff);
        let x = move_in_range_wrap(x, 0..self.width, xdiff);
        Coords::new(y, x)
    }
}

impl IntoIterator for GridBounds {
    type Item = Coords;
    type IntoIter = IterCoords;

    fn into_iter(self) -> IterCoords {
        IterCoords::new(self.height, self.width)
    }
}

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

    pub fn adjacent() -> AdjacentDirs {
        AdjacentDirs::new()
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

    // Returns the direction after rotating 90 degrees to the
    // left/counterclockwise
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
            Direction::Here => Direction::Here,
        }
    }

    // Returns the direction after rotating 90 degrees to the right/clockwise
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
            Direction::Here => Direction::Here,
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::NorthWest => Direction::SouthEast,
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::West => Direction::East,
            Direction::Here => Direction::Here,
            Direction::East => Direction::West,
            Direction::SouthWest => Direction::NorthEast,
            Direction::South => Direction::North,
            Direction::SouthEast => Direction::NorthWest,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

#[cfg(test)]
mod tests {
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
    fn test_row_slice() {
        let gr = Grid::<i32>::parse_words(concat!(
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        ))
        .unwrap();
        assert_eq!(
            gr.row_slice(2..),
            Grid {
                data: vec![
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ]
            }
        );
    }

    #[test]
    fn test_column_slice() {
        let gr = Grid::<i32>::parse_words(concat!(
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        ))
        .unwrap();
        assert_eq!(
            gr.column_slice(..3),
            Grid {
                data: vec![
                    vec![22, 13, 17],
                    vec![8, 2, 23],
                    vec![21, 9, 14],
                    vec![6, 10, 3],
                    vec![1, 12, 20],
                ]
            }
        );
    }
}
