use adventutil::gridgeom::{points_added, Point, PointsAdded, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    fn domove(&mut self, i: Instruction) -> PointsAdded {
        let (facing, dist) = match i {
            Instruction::Left(d) => (self.facing.turn_left(), d),
            Instruction::Right(d) => (self.facing.turn_right(), d),
        };
        let r = points_added(self.pos, facing * dist).unwrap();
        self.pos += facing * dist;
        self.facing = facing;
        r
    }
}

fn solve(input: Input) -> i32 {
    let mut pos = Position::new();
    let mut seen = HashSet::from([pos.pos]);
    for i in input.parse_csv_line::<Instruction>() {
        for p in pos.domove(i) {
            if !seen.insert(p) {
                return (p - Point::ORIGIN).taxicab_len();
            }
        }
    }
    panic!("No location visited twice");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solve(Input::from("R8, R4, R4, R8")), 4);
    }
}
