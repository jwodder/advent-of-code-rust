use adventutil::Input;
use adventutil::numtheory::crt;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Disc {
    positions: i64,
    initial: i64,
}

impl std::str::FromStr for Disc {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Disc, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Disc #")?;
        let _ = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("has ")?;
        let positions = parser.parse_to::<i64, _>(Token::Whitespace)?;
        parser.skip("positions; at time=0, it is at position ")?;
        let initial = parser.parse_to::<i64, _>('.')?;
        parser.eof()?;
        Ok(Disc { positions, initial })
    }
}

fn solve(input: Input) -> i64 {
    crt(input
        .parse_lines::<Disc>()
        .zip(1..)
        .map(|(Disc { positions, initial }, i)| ((-i - initial).rem_euclid(positions), positions)))
    .unwrap()
    .0
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "Disc #1 has 5 positions; at time=0, it is at position 4.\n",
            "Disc #2 has 2 positions; at time=0, it is at position 1.\n",
        ));
        assert_eq!(solve(input), 5);
    }
}
