use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::cmp::max;
use std::str::FromStr;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn ribbon_needed(&self) -> u32 {
        let ribbon = (self.length + self.width + self.height
            - max(max(self.length, self.width), self.height))
            * 2;
        let bow = self.length * self.width * self.height;
        ribbon + bow
    }
}

impl FromStr for Present {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Present, ParseError> {
        let mut parser = PullParser::new(s);
        let length = parser.parse_to::<u32, _>('x')?;
        let width = parser.parse_to::<u32, _>('x')?;
        let height = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(Present {
            length,
            width,
            height,
        })
    }
}

fn solve(input: Input) -> u32 {
    input
        .parse_lines::<Present>()
        .map(|p| p.ribbon_needed())
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let present = "2x3x4".parse::<Present>().unwrap();
        assert_eq!(present.ribbon_needed(), 34);
    }

    #[test]
    fn test_example2() {
        let present = "1x1x10".parse::<Present>().unwrap();
        assert_eq!(present.ribbon_needed(), 14);
    }
}
