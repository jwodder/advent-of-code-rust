use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Scanner {
    depth: usize,
    range: usize,
}

impl std::str::FromStr for Scanner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Scanner, ParseError> {
        let mut parser = PullParser::new(s);
        let depth = parser.parse_to::<usize, _>(": ")?;
        let range = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Scanner { depth, range })
    }
}

fn solve(input: Input) -> usize {
    input
        .parse_lines::<Scanner>()
        .map(|scnr| {
            let pos = scnr.depth % (scnr.range * 2 - 2);
            if pos == 0 { scnr.depth * scnr.range } else { 0 }
        })
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from("0: 3\n1: 2\n4: 4\n6: 4\n");
        assert_eq!(solve(input), 24);
    }
}
