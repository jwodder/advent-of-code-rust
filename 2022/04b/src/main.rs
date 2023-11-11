use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{ranges_overlap, Input};
use std::ops::RangeInclusive;
use std::str::FromStr;

struct RangePair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl RangePair {
    fn is_self_overlapping(&self) -> bool {
        ranges_overlap(self.first.clone(), self.second.clone())
    }
}

impl FromStr for RangePair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RangePair, ParseError> {
        let mut parser = PullParser::new(s);
        let start1 = parser.parse_to::<u32, _>('-')?;
        let end1 = parser.parse_to::<u32, _>(',')?;
        let start2 = parser.parse_to::<u32, _>('-')?;
        let end2 = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(RangePair {
            first: start1..=end1,
            second: start2..=end2,
        })
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
    fn test_example1() {
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
