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
    pos: (i32, i32),
    facing: (i32, i32),
}

impl Position {
    fn new() -> Position {
        Position {
            pos: (0, 0),
            facing: (0, 1),
        }
    }

    fn domove(self, i: Instruction) -> Position {
        let (face_x, face_y, dist) = match i {
            // Rotation matrix: [[0, -1], [1, 0]]
            Instruction::Left(d) => (-self.facing.1, self.facing.0, d),
            // Rotation matrix: [[0, 1], [-1, 0]]
            Instruction::Right(d) => (self.facing.1, -self.facing.0, d),
        };
        Position {
            pos: (self.pos.0 + face_x * dist, self.pos.1 + face_y * dist),
            facing: (face_x, face_y),
        }
    }

    fn distance(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

fn solve(input: Input) -> i32 {
    input
        .parse_csv_line::<Instruction>()
        .into_iter()
        .fold(Position::new(), |p, i| p.domove(i))
        .distance()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
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
