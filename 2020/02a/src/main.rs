use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Password {
    char_qty: RangeInclusive<usize>,
    char_c: char,
    #[allow(clippy::struct_field_names)]
    password: String,
}

impl Password {
    fn valid(&self) -> bool {
        self.char_qty
            .contains(&self.password.chars().filter(|&c| c == self.char_c).count())
    }
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Password, ParseError> {
        let mut parser = PullParser::new(s);
        let start = parser.parse_to::<usize, _>('-')?;
        let end = parser.parse_to::<usize, _>(Token::Whitespace)?;
        let char_c = parser.parse_to::<char, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let password = parser.parse_to::<String, _>(Token::Eof)?;
        Ok(Password {
            char_qty: start..=end,
            char_c,
            password,
        })
    }
}

fn solve(input: Input) -> usize {
    input
        .parse_lines::<Password>()
        .filter(Password::valid)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1-3 a: abcde", true)]
    #[case("1-3 b: cdefg", false)]
    #[case("2-9 c: ccccccccc", true)]
    fn test_valid(#[case] password: Password, #[case] valid: bool) {
        assert_eq!(password.valid(), valid);
    }
}
