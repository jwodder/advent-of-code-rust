use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
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
    digits.len() == 6
        && digits.windows(2).all(|w| w[0] <= w[1])
        && digits
            .into_iter()
            .group_by(|&c| c)
            .into_iter()
            .any(|(_, gr)| gr.count() == 2)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(112233, true)]
    #[case(123444, false)]
    #[case(111122, true)]
    fn test_valid(#[case] n: u32, #[case] v: bool) {
        assert_eq!(valid(n), v);
    }
}
