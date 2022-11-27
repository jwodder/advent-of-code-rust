use adventutil::Input;
use either::Either;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Line {
    Horizontal { y: i32, x1: i32, x2: i32 },
    Vertical { x: i32, y1: i32, y2: i32 },
    Diagonal { x1: i32, y1: i32, x2: i32, y2: i32 },
}

impl Line {
    fn points(self) -> impl Iterator<Item = (i32, i32)> {
        match self {
            Line::Horizontal { y, x1, x2 } => {
                Either::Left((min(x1, x2)..=max(x1, x2)).map(move |x| (y, x)))
            }
            Line::Vertical { x, y1, y2 } => Either::Right(Either::Left(
                (min(y1, y2)..=max(y1, y2)).map(move |y| (y, x)),
            )),
            Line::Diagonal { x1, y1, x2, y2 } => {
                let xdiff = if x1 < x2 { 1 } else { -1 };
                let ydiff = if y1 < y2 { 1 } else { -1 };
                let n = (x1 - x2).abs();
                Either::Right(Either::Right(
                    (0..=n).map(move |i| (y1 + i * ydiff, x1 + i * xdiff)),
                ))
            }
        }
    }
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Line, ParseLineError> {
        let (start, end) = s
            .split_once("->")
            .ok_or_else(|| ParseLineError::Syntax(s.to_string()))?;
        let (sx1, sy1) = start
            .trim()
            .split_once(',')
            .ok_or_else(|| ParseLineError::Syntax(s.to_string()))?;
        let (sx2, sy2) = end
            .trim()
            .split_once(',')
            .ok_or_else(|| ParseLineError::Syntax(s.to_string()))?;
        let x1 = sx1.parse::<i32>()?;
        let y1 = sy1.parse::<i32>()?;
        let x2 = sx2.parse::<i32>()?;
        let y2 = sy2.parse::<i32>()?;
        if x1 == x2 {
            Ok(Line::Vertical { x: x1, y1, y2 })
        } else if y1 == y2 {
            Ok(Line::Horizontal { y: y1, x1, x2 })
        } else if x1.abs_diff(x2) == y1.abs_diff(y2) {
            Ok(Line::Diagonal { x1, y1, x2, y2 })
        } else {
            Err(ParseLineError::Slanted(s.to_string()))
        }
    }
}

#[derive(Debug, Error)]
enum ParseLineError {
    #[error("Malformed line: {0:?}")]
    Syntax(String),
    #[error("Slanted line: {0:?}")]
    Slanted(String),
    #[error("Invalid integer in line: {0}")]
    BadInt(#[from] ParseIntError),
}

fn main() {
    let lines = Input::from_env().parse_lines::<Line>();
    println!("{}", count_overlaps(lines));
}

fn count_overlaps<I: IntoIterator<Item = Line>>(lines: I) -> usize {
    let mut counter = HashMap::new();
    for ln in lines {
        for p in ln.points() {
            *counter.entry(p).or_insert(0) += 1;
        }
    }
    counter.into_values().filter(|qty| qty > &1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let lines = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .into_iter()
        .map(|s| s.parse::<Line>().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(count_overlaps(lines), 12);
    }
}