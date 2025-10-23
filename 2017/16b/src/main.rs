// PROBLEM: I think the "Partner" move may preclude treating this as a
// "permutation exponentiation" problem.
use adventutil::Input;
use std::collections::VecDeque;
use std::fmt;
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Permutation(Vec<u8>);

impl Permutation {
    fn from_permuted(line: VecDeque<u8>) -> Permutation {
        let mut perm = vec![0; line.len()];
        for i in 0..line.len() {
            perm[usize::from(line[i])] = u8::try_from(i).unwrap();
        }
        Permutation(perm)
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.0 {
            write!(f, "{}", char::from(i + 0x61))?;
        }
        Ok(())
    }
}

impl std::ops::MulAssign<&Permutation> for Permutation {
    fn mul_assign(&mut self, rhs: &Permutation) {
        self.0 = rhs.0.iter().map(|&i| self.0[usize::from(i)]).collect();
    }
}

fn solve(input: Input, dancers: u8) -> String {
    let mut line = VecDeque::from_iter(0..dancers);
    for m in input.parse_csv_line::<Move>() {
        m.apply(&mut line);
    }
    let mut p = Permutation::from_permuted(line);
    let mut n = 1_000_000_000;
    while n & 1 == 0 {
        p *= &p.clone();
        n >>= 1;
    }
    let mut agg = p.clone();
    p *= &agg;
    n >>= 1;
    while n > 0 {
        if n & 1 == 1 {
            agg *= &p;
        }
        p *= &p.clone();
        n >>= 1;
    }
    let mut line2 = vec!['a'; usize::from(dancers)];
    for i in 0..dancers {
        line2[usize::from(p.0[usize::from(i)])] = char::from(0x61 + i);
    }
    String::from_iter(line2)
}

fn main() {
    println!("{}", solve(Input::from_env(), 16));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_permuted() {
        let line = VecDeque::from([1, 0, 3, 4, 2]);
        assert_eq!(Permutation::from_permuted(line).to_string(), "baecd");
    }

    #[test]
    fn mul1() {
        let mut p1 = Permutation(vec![1, 0, 2, 3, 4]);
        let p2 = Permutation(vec![0, 1, 3, 4, 2]);
        p1 *= &p2;
        assert_eq!(p1.to_string(), "badec");
    }

    #[test]
    fn mul2() {
        let mut p1 = Permutation(vec![2, 0, 1, 3, 4]); // (0 2 1)
        let p2 = Permutation(vec![0, 3, 1, 2, 4]); // (1 3 2)
        p1 *= &p2;
        // p1 = (0 2)(1 3) = [2, 3, 0, 1, 4]
        assert_eq!(p1.to_string(), "cdabe");
    }
}
