use std::iter::FusedIterator;
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

    pub fn map<U, F>(self, mut f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        let mut data = Vec::with_capacity(self.data.len());
        for row in self.data {
            let mut new_row = Vec::with_capacity(row.len());
            for value in row {
                new_row.push(f(value));
            }
            data.push(new_row);
        }
        Grid { data }
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
    type Err = GridParseError<<T as FromStr>::Err>;

    fn from_str(s: &str) -> Result<Grid<T>, Self::Err> {
        Grid::try_from(
            s.lines()
                .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )?
        .try_map(|s| s.parse::<T>())
        .map_err(GridParseError::Parse)
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
pub enum GridParseError<E> {
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
}
