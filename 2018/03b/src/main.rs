use adventutil::Input;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use thiserror::Error;

struct Claim {
    id: String,
    left_margin: usize,
    top_margin: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn yrange(&self) -> Range<usize> {
        self.top_margin..(self.top_margin + self.height)
    }

    fn xrange(&self) -> Range<usize> {
        self.left_margin..(self.left_margin + self.width)
    }

    fn overlaps(&self, other: &Claim) -> bool {
        fn ranges_overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
            r1.start.max(r2.start) < r1.end.min(r2.end)
            //r2.start < r1.end && r1.start < r2.end
        }

        ranges_overlap(self.xrange(), other.xrange())
            && ranges_overlap(self.yrange(), other.yrange())
    }
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Claim, ParseError> {
        let (id, measurements) = s
            .split_once('@')
            .ok_or_else(|| ParseError::Syntax(s.into()))?;
        let id = id.trim().trim_start_matches('#').to_string();
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

#[derive(Debug, Error)]
enum ParseError {
    #[error("Malformed claim: {0:?}")]
    Syntax(String),
    #[error("Invalid integer: {0:?}")]
    InvalidInteger(#[from] ParseIntError),
}

fn non_overlapping<I: IntoIterator<Item = Claim>>(claims: I) -> String {
    let claims = claims.into_iter().collect::<Vec<_>>();
    let mut disqualified = HashSet::new();
    for i in 0..claims.len() {
        let mut does_overlap = disqualified.contains(&i);
        for j in (i + 1)..claims.len() {
            if claims[i].overlaps(&claims[j]) {
                disqualified.insert(j);
                does_overlap = true;
            }
        }
        if !does_overlap {
            return claims[i].id.clone();
        }
    }
    panic!("No non-overlapping claim");
}

fn main() {
    println!(
        "{}",
        non_overlapping(Input::from_env().parse_lines::<Claim>())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let claims = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .map(|s| s.parse::<Claim>().unwrap());
        assert_eq!(non_overlapping(claims), "3");
    }
}
