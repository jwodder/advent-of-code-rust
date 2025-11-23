use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    start: u32,
    end: u32, // inclusive
}

impl std::str::FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Range, ParseError> {
        let mut parser = PullParser::new(s);
        let start = parser.parse_to::<u32, _>('-')?;
        let end = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(Range { start, end })
    }
}

fn solve(input: Input) -> u32 {
    let mut ranges = input.parse_lines::<Range>().collect::<Vec<_>>();
    ranges.sort_unstable();
    let mut iter = ranges.into_iter();
    let mut leading = iter.next().unwrap();
    if leading.start > 0 {
        return 0;
    }
    for r in iter {
        if leading.end + 1 >= r.start {
            leading.end = leading.end.max(r.end);
        } else {
            break;
        }
    }
    leading.end + 1
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("5-8\n0-2\n4-7\n");
        assert_eq!(solve(input), 3);
    }
}
