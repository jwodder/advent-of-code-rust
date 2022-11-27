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
            .map(|p| p.ribbon_needed())
            .sum::<u32>()
    );
}

#[cfg(test)]
mod test {
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
