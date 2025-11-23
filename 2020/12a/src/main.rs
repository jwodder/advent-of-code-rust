use adventutil::Input;
use adventutil::gridgeom::{Point, Vector};
use adventutil::pullparser::ParseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        use Instruction::*;
        let inst = s
            .get(0..1)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?;
        let arg = s
            .get(1..)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?
            .parse::<i32>()?;
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
    let mut facing = Vector::EAST;
    let mut pos = Point::ORIGIN;
    for instruction in input.parse_lines::<Instruction>() {
        use Instruction::*;
        match instruction {
            North(dist) => pos += Vector::NORTH * dist,
            South(dist) => pos += Vector::SOUTH * dist,
            East(dist) => pos += Vector::EAST * dist,
            West(dist) => pos += Vector::WEST * dist,
            Left(angle) => {
                for _ in 0..(angle / 90) {
                    facing = facing.turn_left();
                }
            }
            Right(angle) => {
                for _ in 0..(angle / 90) {
                    facing = facing.turn_right();
                }
            }
            Forward(dist) => {
                pos += facing * dist;
            }
        }
    }
    (pos - Point::ORIGIN).taxicab_len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("F10\nN3\nF7\nR90\nF11\n");
        assert_eq!(solve(input), 25);
    }
}
