use adventutil::Input;
use std::cmp::max;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

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
        let mut sides = s.split('x');
        let length = sides.next().ok_or(ParseError::Syntax)?.parse::<u32>()?;
        let width = sides.next().ok_or(ParseError::Syntax)?.parse::<u32>()?;
        let height = sides.next().ok_or(ParseError::Syntax)?.parse::<u32>()?;
        if sides.next().is_some() {
            Err(ParseError::Syntax)
        } else {
            Ok(Present {
                length,
                width,
                height,
            })
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Malformed input")]
    Syntax,
    #[error("Invalid integer: {0}")]
    InvalidInteger(#[from] ParseIntError),
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
