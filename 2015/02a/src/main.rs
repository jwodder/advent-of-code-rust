use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::cmp::max;
use std::str::FromStr;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn paper_needed(&self) -> u32 {
        let area =
            2 * (self.length * self.width + self.width * self.height + self.height * self.length);
        let smallest = (self.length * self.width * self.height)
            / max(max(self.length, self.width), self.height);
        area + smallest
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

fn main() {
    println!(
        "{}",
        Input::from_env()
            .parse_lines::<Present>()
            .map(|p| p.paper_needed())
            .sum::<u32>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let present = "2x3x4".parse::<Present>().unwrap();
        assert_eq!(present.paper_needed(), 58);
    }

    #[test]
    fn test_example2() {
        let present = "1x1x10".parse::<Present>().unwrap();
        assert_eq!(present.paper_needed(), 43);
    }
}
