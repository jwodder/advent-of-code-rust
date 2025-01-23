use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockRange {
    depth: usize,
    modulus: usize,
}

impl BlockRange {
    fn passes(&self, t: usize) -> bool {
        (t + self.depth) % self.modulus != 0
    }
}

impl From<Scanner> for BlockRange {
    fn from(value: Scanner) -> BlockRange {
        let modulus = value.range * 2 - 2;
        BlockRange {
            depth: value.depth,
            modulus,
        }
    }
}

fn solve(input: Input) -> usize {
    let blocks = input
        .parse_lines::<Scanner>()
        .map(BlockRange::from)
        .collect::<Vec<_>>();
    (0..)
        .find(|&t| blocks.iter().all(|br| br.passes(t)))
        .unwrap()
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
        assert_eq!(solve(input), 10);
    }
}
