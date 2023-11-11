use adventutil::area::Area;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
use std::str::FromStr;

struct Claim {
    id: String,
    left_margin: usize,
    top_margin: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn area(&self) -> Area<usize> {
        Area {
            start_x: self.left_margin,
            end_x: self.left_margin + self.width,
            start_y: self.top_margin,
            end_y: self.top_margin + self.height,
        }
    }

    fn overlaps(&self, other: &Claim) -> bool {
        self.area().intersects(other.area())
    }
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Claim, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip('#')?;
        let id = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip('@')?;
        parser.skip(Token::Whitespace)?;
        let left_margin = parser.parse_to::<usize, _>(',')?;
        let top_margin = parser.parse_to::<usize, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let width = parser.parse_to::<usize, _>('x')?;
        let height = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Claim {
            id,
            left_margin,
            top_margin,
            width,
            height,
        })
    }
}

fn solve(input: Input) -> String {
    let claims = input.parse_lines::<Claim>().collect::<Vec<_>>();
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
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n");
        assert_eq!(solve(input), "3");
    }
}
