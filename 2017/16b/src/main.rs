#![allow(clippy::cast_possible_truncation)]
// Trying to solve this by just composing all the permutations in order and
// then raising the result to the power of a billion doesn't work, due to the
// "Partner" moves not evaluating to the same permutations on subsequent
// dances.  Instead, the "Partner" moves should be treated as transpositions
// applied before any dance starts; the correct algorithm is thus:
//
// - Compute $p$, the product of all "Partner" moves (with the first move on
//   the left end), with each such move interpreted as a transposition of
//   ord(A) and ord(B)
// - Compute $s$, the product of all other moves (with the first move on the
//   right)
// - Compute $r = s^n p^n$ where $n$ is a billion
// - Permute "a..p" by rearranging the indices according to $r$
use adventutil::Input;
use std::fmt;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
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
        (Some(ch), None) if ch.is_ascii_lowercase() => Ok(char_to_ord(ch)),
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
    const DEGREE: usize = 16;

    fn permute(&self, slice: &[char]) -> Vec<char> {
        let mut out = vec!['a'; slice.len()];
        for (i, &c) in slice.iter().enumerate() {
            out[usize::from(self.0[i])] = c;
        }
        out
    }

    fn pow(&self, mut n: usize) -> Permutation {
        if n == 0 {
            return Permutation::default();
        }
        let mut p = self.clone();
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
        agg
    }
}

impl Default for Permutation {
    fn default() -> Permutation {
        Permutation((0..(Permutation::DEGREE as u8)).collect())
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &i in &self.0 {
            write!(f, "{}", ord_to_char(i))?;
        }
        Ok(())
    }
}

impl From<Move> for Permutation {
    fn from(m: Move) -> Permutation {
        match m {
            Move::Spin(offset) => {
                let mut p = Permutation::default();
                p.0.rotate_left(offset);
                p
            }
            Move::Exchange(a, b) => {
                let mut p = Permutation::default();
                p.0.swap(a, b);
                p
            }
            Move::Partner(a, b) => {
                let mut p = Permutation::default();
                p.0.swap(usize::from(a), usize::from(b));
                p
            }
        }
    }
}

impl std::ops::Mul for &Permutation {
    type Output = Permutation;

    fn mul(self, rhs: &Permutation) -> Permutation {
        Permutation(rhs.0.iter().map(|&i| self.0[usize::from(i)]).collect())
    }
}

impl std::ops::MulAssign<&Permutation> for Permutation {
    fn mul_assign(&mut self, rhs: &Permutation) {
        self.0 = rhs.0.iter().map(|&i| self.0[usize::from(i)]).collect();
    }
}

fn char_to_ord(c: char) -> u8 {
    u8::try_from(u32::from(c) - 0x61).unwrap()
}

fn ord_to_char(i: u8) -> char {
    char::from(0x61 + i)
}

fn solve(input: Input) -> String {
    let mut partners = Permutation::default();
    let mut sigma = Permutation::default();
    for m in input.parse_csv_line::<Move>() {
        let p = Permutation::from(m);
        if matches!(m, Move::Partner(..)) {
            partners *= &p;
        } else {
            sigma = &p * &sigma;
        }
    }
    let n = 1_000_000_000;
    let res = &sigma.pow(n) * &partners.pow(n);
    let line = (0..(Permutation::DEGREE as u8))
        .map(ord_to_char)
        .collect::<Vec<_>>();
    String::from_iter(res.permute(&line))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

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
