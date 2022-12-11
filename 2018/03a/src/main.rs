use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::{Itertools, Product};
use std::ops::Range;
use std::str::FromStr;

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

impl IntoIterator for Claim {
    type Item = (usize, usize);
    type IntoIter = Product<Range<usize>, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (self.left_margin..(self.left_margin + self.width))
            .cartesian_product(self.top_margin..(self.top_margin + self.height))
    }
}

fn solve(input: Input) -> usize {
    input
        .parse_lines::<Claim>()
        .flat_map(|c| c.into_iter())
        .collect::<Counter<(usize, usize)>>()
        .into_values()
        .filter(|&qty| qty >= 2)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n");
        assert_eq!(solve(input), 4);
    }
}
