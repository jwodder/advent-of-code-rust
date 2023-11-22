use adventutil::gridgeom::{Point, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

enum Instruction {
    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s.trim());
        if parser.skip("L").is_ok() {
            let dist = parser.parse_to::<i32, _>(Token::Eof)?;
            Ok(Instruction::Left(dist))
        } else if parser.skip("R").is_ok() {
            let dist = parser.parse_to::<i32, _>(Token::Eof)?;
            Ok(Instruction::Right(dist))
        } else {
            Err(ParseError::InvalidToken(s.into()))
        }
    }
}

struct Position {
    pos: Point,
    facing: Vector,
}

impl Position {
    fn new() -> Position {
        Position {
            pos: Point::ORIGIN,
            facing: Vector::NORTH,
        }
    }

    fn domove(self, i: Instruction) -> Position {
        let (facing, dist) = match i {
            Instruction::Left(d) => (self.facing.turn_left(), d),
            Instruction::Right(d) => (self.facing.turn_right(), d),
        };
        Position {
            pos: self.pos + facing * dist,
            facing,
        }
    }

    fn distance(&self) -> i32 {
        (self.pos - Point::ORIGIN).taxicab_len()
    }
}

fn solve(input: Input) -> i32 {
    input
        .parse_csv_line::<Instruction>()
        .into_iter()
        .fold(Position::new(), Position::domove)
        .distance()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("R2, L3", 5)]
    #[case("R2, R2, R2", 2)]
    #[case("R5, L5, R5, R3", 12)]
    fn test_solve(#[case] s: &'static str, #[case] blocks: i32) {
        assert_eq!(solve(Input::from(s)), blocks);
    }
}
