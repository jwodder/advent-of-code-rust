use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::ops::RangeInclusive;

fn parse_range(s: &str) -> Result<RangeInclusive<u32>, ParseError> {
    let mut parser = PullParser::new(s);
    let start = parser.parse_to::<u32, _>('-')?;
    let end = parser.parse_to::<u32, _>(Token::Eof)?;
    Ok(start..=end)
}

fn solve(input: Input) -> usize {
    let range = parse_range(input.read().trim()).expect("Parse error");
    range.filter(|&n| valid(n)).count()
}

fn valid(n: u32) -> bool {
    let digits = n.to_string().chars().collect::<Vec<_>>();
    if digits.len() != 6 {
        return false;
    }
    let mut has_double = false;
    for w in digits.windows(2) {
        if w[0] == w[1] {
            has_double = true;
        }
        if w[0] > w[1] {
            return false;
        }
    }
    has_double
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(122345, true)]
    #[case(111123, true)]
    #[case(111111, true)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn test_valid(#[case] n: u32, #[case] v: bool) {
        assert_eq!(valid(n), v);
    }
}
