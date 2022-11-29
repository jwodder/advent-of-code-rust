use adventutil::counter::Counter;
use adventutil::Input;
use itertools::{Itertools, Product};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use thiserror::Error;

struct Claim {
    #[allow(unused)]
    id: String,
    left_margin: usize,
    top_margin: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Claim, ParseError> {
        let (id, measurements) = s
            .split_once('@')
            .ok_or_else(|| ParseError::Syntax(s.into()))?;
        let id = id.trim().to_string();
        let (margins, dimens) = measurements
            .trim()
            .split_once(": ")
            .ok_or_else(|| ParseError::Syntax(s.into()))?;
        let (left_margin, top_margin) = margins
            .split_once(',')
            .ok_or_else(|| ParseError::Syntax(s.into()))?;
        let left_margin = left_margin.parse::<usize>()?;
        let top_margin = top_margin.parse::<usize>()?;
        let (width, height) = dimens
            .split_once('x')
            .ok_or_else(|| ParseError::Syntax(s.into()))?;
        let width = width.parse::<usize>()?;
        let height = height.parse::<usize>()?;
        Ok(Claim {
            id,
            left_margin,
            top_margin,
            width,
            height,
        })
    }
}

impl IntoIterator for Claim {
    type Item = (usize, usize);
    type IntoIter = Product<Range<usize>, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (self.left_margin..(self.left_margin + self.width))
            .cartesian_product(self.top_margin..(self.top_margin + self.height))
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Malformed claim: {0:?}")]
    Syntax(String),
    #[error("Invalid integer: {0:?}")]
    InvalidInteger(#[from] ParseIntError),
}

fn overlaps<I: IntoIterator<Item = Claim>>(claims: I) -> usize {
    claims
        .into_iter()
        .flat_map(|c| c.into_iter())
        .collect::<Counter<(usize, usize)>>()
        .into_values()
        .filter(|&qty| qty >= 2)
        .count()
}

fn main() {
    println!("{}", overlaps(Input::from_env().parse_lines::<Claim>()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let claims = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .map(|s| s.parse::<Claim>().unwrap());
        assert_eq!(overlaps(claims), 4);
    }
}
