use adventutil::Input;
use std::collections::VecDeque;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Move {
    fn apply(self, line: &mut VecDeque<u8>) {
        match self {
            Move::Spin(offset) => line.rotate_right(offset),
            Move::Exchange(a, b) => line.swap(a, b),
            Move::Partner(a, b) => {
                let i = line.iter().position(|&d| d == a).unwrap();
                let j = line.iter().position(|&d| d == b).unwrap();
                line.swap(i, j);
            }
        }
    }
}

impl std::str::FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Move, ParseMoveError> {
        match s.split_at_checked(1) {
            Some(("s", rest)) => {
                let offset = rest.parse::<usize>()?;
                Ok(Move::Spin(offset))
            }
            Some(("x", rest)) => {
                let (a, b) = rest
                    .split_once('/')
                    .ok_or_else(|| ParseMoveError::MissingSlash(s.to_owned()))?;
                let a = a.parse::<usize>()?;
                let b = b.parse::<usize>()?;
                Ok(Move::Exchange(a, b))
            }
            Some(("p", rest)) => {
                let (a, b) = rest
                    .split_once('/')
                    .ok_or_else(|| ParseMoveError::MissingSlash(s.to_owned()))?;
                let a = parse_dancer(a)?;
                let b = parse_dancer(b)?;
                Ok(Move::Partner(a, b))
            }
            Some((start, _)) => Err(ParseMoveError::InvalidType(start.to_owned())),
            None => Err(ParseMoveError::Empty),
        }
    }
}

fn parse_dancer(s: &str) -> Result<u8, ParseMoveError> {
    let mut chars = s.chars();
    let c1 = chars.next();
    let c2 = chars.next();
    match (c1, c2) {
        (Some(ch), None) if ch.is_ascii_lowercase() => {
            Ok(u8::try_from(u32::from(ch) - 0x61).unwrap())
        }
        _ => Err(ParseMoveError::InvalidDancer(s.to_owned())),
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
enum ParseMoveError {
    #[error("empty dance move")]
    Empty,
    #[error("invalid dance move type: {0:?}")]
    InvalidType(String),
    #[error("dance move {0:?} is missing required slash")]
    MissingSlash(String),
    #[error("invalid dancer: {0:?}")]
    InvalidDancer(String),
    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

fn solve(input: Input, dancers: u8) -> String {
    let mut line = VecDeque::from_iter(0..dancers);
    for m in input.parse_csv_line::<Move>() {
        m.apply(&mut line);
    }
    line.into_iter().map(|i| char::from(i + 0x61)).collect()
}

fn main() {
    println!("{}", solve(Input::from_env(), 16));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("s1,x3/4,pe/b");
        assert_eq!(solve(input, 5), "baedc");
    }
}
