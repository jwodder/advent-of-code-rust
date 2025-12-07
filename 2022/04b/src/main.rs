use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::ranges::{parse_range, ranges_overlap};
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Eq, PartialEq)]
struct RangePair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl RangePair {
    fn is_self_overlapping(&self) -> bool {
        ranges_overlap(self.first.clone(), self.second.clone())
    }
}

impl std::str::FromStr for RangePair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RangePair, ParseError> {
        let mut parser = PullParser::new(s);
        let first = parse_range::<u32>(parser.scan_to(',')?)?;
        let second = parse_range::<u32>(parser.scan_to(Token::Eof)?)?;
        Ok(RangePair { first, second })
    }
}

fn solve(input: Input) -> usize {
    input
        .parse_lines::<RangePair>()
        .filter(RangePair::is_self_overlapping)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8\n",
        ));
        assert_eq!(solve(input), 4);
    }
}
