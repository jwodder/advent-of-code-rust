use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

struct Password {
    pos1: usize,
    pos2: usize,
    char_c: char,
    password: String,
}

impl Password {
    fn valid(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|&(i, c)| (i == self.pos1 - 1 || i == self.pos2 - 1) && c == self.char_c)
            .count()
            == 1
    }
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Password, ParseError> {
        let mut parser = PullParser::new(s);
        let pos1 = parser.parse_to::<usize, _>('-')?;
        let pos2 = parser.parse_to::<usize, _>(Token::Whitespace)?;
        let char_c = parser.parse_to::<char, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let password = parser.parse_to::<String, _>(Token::Eof)?;
        Ok(Password {
            pos1,
            pos2,
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
    #[case("2-9 c: ccccccccc", false)]
    fn test_valid(#[case] password: Password, #[case] valid: bool) {
        assert_eq!(password.valid(), valid);
    }
}
