use adventutil::gridgeom::{Point, Vector};
use adventutil::pullparser::ParseError;
use adventutil::Input;
use std::str::FromStr;

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let inst = s
            .get(0..1)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?;
        let arg = s
            .get(1..)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?
            .parse::<i32>()?;
        use Instruction::*;
        match inst {
            "N" => Ok(North(arg)),
            "S" => Ok(South(arg)),
            "E" => Ok(East(arg)),
            "W" => Ok(West(arg)),
            "L" if arg % 90 == 0 => Ok(Left(arg)),
            "R" if arg % 90 == 0 => Ok(Right(arg)),
            "F" => Ok(Forward(arg)),
            _ => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn solve(input: Input) -> i32 {
    let mut ship = Point::ORIGIN;
    let mut waypoint = Vector { x: 10, y: 1 };
    for instruction in input.parse_lines::<Instruction>() {
        use Instruction::*;
        match instruction {
            North(dist) => waypoint += Vector::NORTH * dist,
            South(dist) => waypoint += Vector::SOUTH * dist,
            East(dist) => waypoint += Vector::EAST * dist,
            West(dist) => waypoint += Vector::WEST * dist,
            Left(angle) => {
                for _ in 0..(angle / 90) {
                    waypoint = waypoint.turn_left();
                }
            }
            Right(angle) => {
                for _ in 0..(angle / 90) {
                    waypoint = waypoint.turn_right();
                }
            }
            Forward(dist) => ship += waypoint * dist,
        }
    }
    (ship - Point::ORIGIN).taxicab_len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("F10\nN3\nF7\nR90\nF11\n");
        assert_eq!(solve(input), 286);
    }
}
