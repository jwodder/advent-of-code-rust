use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use either::Either;
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Location(i32, i32);

impl Location {
    fn points_from(&self, other: Location) -> impl Iterator<Item = Location> {
        if self.0 == other.0 && self.1 != other.1 {
            let start = other.1;
            Either::Left(
                ((self.1.min(other.1))..(self.1.max(other.1)))
                    .filter(move |&y| y != start)
                    .map(move |y| Location(other.0, y)),
            )
        } else if self.0 != other.0 && self.1 == other.1 {
            let start = other.0;
            Either::Right(
                ((self.0.min(other.0))..(self.0.max(other.0)))
                    .filter(move |&x| x != start)
                    .map(move |x| Location(x, other.1)),
            )
        } else {
            panic!("No rectilinear line from {self:?} to {other:?}");
        }
    }

    fn distance(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    pos: Location,
    facing: (i32, i32),
}

impl Position {
    fn new() -> Position {
        Position {
            pos: Location(0, 0),
            facing: (0, 1),
        }
    }

    fn domove(&mut self, i: Instruction) -> impl Iterator<Item = Location> {
        let (face_x, face_y, dist) = match i {
            // Rotation matrix: [[0, -1], [1, 0]]
            Instruction::Left(d) => (-self.facing.1, self.facing.0, d),
            // Rotation matrix: [[0, 1], [-1, 0]]
            Instruction::Right(d) => (self.facing.1, -self.facing.0, d),
        };
        let newpos = Location(self.pos.0 + face_x * dist, self.pos.1 + face_y * dist);
        let r = newpos.points_from(self.pos);
        self.pos = newpos;
        self.facing = (face_x, face_y);
        r
    }
}

fn solve(input: Input) -> i32 {
    let mut pos = Position::new();
    let mut seen = HashSet::new();
    for i in input.parse_csv_line::<Instruction>() {
        for p in pos.domove(i) {
            if !seen.insert(p) {
                return p.distance();
            }
        }
    }
    panic!("No location visited twice");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solve(Input::from("R8, R4, R4, R8")), 4);
    }
}
