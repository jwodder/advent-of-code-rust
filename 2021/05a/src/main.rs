use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Line {
    Horizontal { y: u32, x1: u32, x2: u32 },
    Vertical { x: u32, y1: u32, y2: u32 },
    Diagonal { x1: u32, y1: u32, x2: u32, y2: u32 },
}

impl Line {
    fn points(self) -> Box<dyn Iterator<Item = (u32, u32)>> {
        match self {
            Line::Horizontal { y, x1, x2 } => {
                Box::new((min(x1, x2)..=max(x1, x2)).map(move |x| (y, x)))
            }
            Line::Vertical { x, y1, y2 } => {
                Box::new((min(y1, y2)..=max(y1, y2)).map(move |y| (y, x)))
            }
            Line::Diagonal { .. } => unreachable!(),
        }
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Line, ParseError> {
        let mut parser = PullParser::new(s);
        let x1 = parser.parse_to::<u32, _>(',')?;
        let y1 = parser.parse_to::<u32, _>(Token::Whitespace)?;
        parser.skip("->")?;
        parser.skip(Token::Whitespace)?;
        let x2 = parser.parse_to::<u32, _>(',')?;
        let y2 = parser.parse_to::<u32, _>(Token::Eof)?;
        if x1 == x2 {
            Ok(Line::Vertical { x: x1, y1, y2 })
        } else if y1 == y2 {
            Ok(Line::Horizontal { y: y1, x1, x2 })
        } else {
            Ok(Line::Diagonal { x1, y1, x2, y2 })
        }
    }
}

fn solve(input: Input) -> usize {
    let mut counter = Counter::new();
    for ln in input.parse_lines::<Line>() {
        if !matches!(ln, Line::Diagonal { .. }) {
            for p in ln.points() {
                counter.add(p);
            }
        }
    }
    counter.into_values().filter(|&qty| qty > 1).count()
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
            "0,9 -> 5,9\n",
            "8,0 -> 0,8\n",
            "9,4 -> 3,4\n",
            "2,2 -> 2,1\n",
            "7,0 -> 7,4\n",
            "6,4 -> 2,0\n",
            "0,9 -> 2,9\n",
            "3,4 -> 1,4\n",
            "0,0 -> 8,8\n",
            "5,5 -> 8,2\n",
        ));
        assert_eq!(solve(input), 5);
    }
}
