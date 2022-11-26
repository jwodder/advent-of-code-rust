use std::fmt;
use std::str::FromStr;

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

    //pub fn get_wrap(&self, y: isize, x: isize) -> &T {
    //    self.get(y.rem_euclid(self.height()), x.rem_euclid(self.width()))
    //        .unwrap()
    //}
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

impl FromStr for Grid<char> {
    type Err = GridFromError;

    fn from_str(s: &str) -> Result<Grid<char>, GridFromError> {
        s.lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridFromError {
    Empty,
    Ragged,
}

impl fmt::Display for GridFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GridFromError::Empty => write!(f, "Input grid is empty"),
            GridFromError::Ragged => write!(f, "Input grid is ragged/uneven"),
        }
    }
}

impl std::error::Error for GridFromError {}
