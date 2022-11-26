use std::fmt;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
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

    fn ynorth(&self) -> Option<usize> {
        self.y.checked_sub(1)
    }

    fn ynorth_wrap(&self) -> usize {
        self.y
            .checked_sub(1)
            .unwrap_or_else(|| self.grid.height() - 1)
    }

    fn ysouth(&self) -> Option<usize> {
        let y = self.y + 1;
        if y >= self.grid.height() {
            None
        } else {
            Some(y)
        }
    }

    fn ysouth_wrap(&self) -> usize {
        self.ysouth().unwrap_or(0)
    }

    fn xwest(&self) -> Option<usize> {
        self.x.checked_sub(1)
    }

    fn xwest_wrap(&self) -> usize {
        self.x
            .checked_sub(1)
            .unwrap_or_else(|| self.grid.width() - 1)
    }

    fn xeast(&self) -> Option<usize> {
        let x = self.x + 1;
        if x >= self.grid.width() {
            None
        } else {
            Some(x)
        }
    }

    fn xeast_wrap(&self) -> usize {
        self.xeast().unwrap_or(0)
    }

    pub fn north(&self) -> Option<Cell<'_, T>> {
        self.ynorth().and_then(|y| self.grid.get_cell(y, self.x))
    }

    pub fn north_wrap(&self) -> Cell<'_, T> {
        self.grid.get_cell(self.ynorth_wrap(), self.x).unwrap()
    }

    pub fn south(&self) -> Option<Cell<'_, T>> {
        self.ysouth().and_then(|y| self.grid.get_cell(y, self.x))
    }

    pub fn south_wrap(&self) -> Cell<'_, T> {
        self.grid.get_cell(self.ysouth_wrap(), self.x).unwrap()
    }

    pub fn east(&self) -> Option<Cell<'_, T>> {
        self.xeast().and_then(|x| self.grid.get_cell(self.y, x))
    }

    pub fn east_wrap(&self) -> Cell<'_, T> {
        self.grid.get_cell(self.y, self.xeast_wrap()).unwrap()
    }

    pub fn west(&self) -> Option<Cell<'_, T>> {
        self.xwest().and_then(|x| self.grid.get_cell(self.y, x))
    }

    pub fn west_wrap(&self) -> Cell<'_, T> {
        self.grid.get_cell(self.y, self.xwest_wrap()).unwrap()
    }

    pub fn north_east(&self) -> Option<Cell<'_, T>> {
        let y = self.ynorth()?;
        let x = self.xeast()?;
        Some(self.grid.get_cell(y, x).unwrap())
    }

    pub fn north_east_wrap(&self) -> Cell<'_, T> {
        let y = self.ynorth_wrap();
        let x = self.xeast_wrap();
        self.grid.get_cell(y, x).unwrap()
    }

    pub fn north_west(&self) -> Option<Cell<'_, T>> {
        let y = self.ynorth()?;
        let x = self.xwest()?;
        Some(self.grid.get_cell(y, x).unwrap())
    }

    pub fn north_west_wrap(&self) -> Cell<'_, T> {
        let y = self.ynorth_wrap();
        let x = self.xwest_wrap();
        self.grid.get_cell(y, x).unwrap()
    }

    pub fn south_east(&self) -> Option<Cell<'_, T>> {
        let y = self.ysouth()?;
        let x = self.xeast()?;
        Some(self.grid.get_cell(y, x).unwrap())
    }

    pub fn south_east_wrap(&self) -> Cell<'_, T> {
        let y = self.ysouth_wrap();
        let x = self.xeast_wrap();
        self.grid.get_cell(y, x).unwrap()
    }

    pub fn south_west(&self) -> Option<Cell<'_, T>> {
        let y = self.ysouth()?;
        let x = self.xwest()?;
        Some(self.grid.get_cell(y, x).unwrap())
    }

    pub fn south_west_wrap(&self) -> Cell<'_, T> {
        let y = self.ysouth_wrap();
        let x = self.xwest_wrap();
        self.grid.get_cell(y, x).unwrap()
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
}
