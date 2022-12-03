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
    let (mut ship_x, mut ship_y) = (0, 0);
    let (mut waypoint_x, mut waypoint_y) = (10, 1);
    for instruction in input.parse_lines::<Instruction>() {
        use Instruction::*;
        match instruction {
            North(dist) => waypoint_y += dist,
            South(dist) => waypoint_y -= dist,
            East(dist) => waypoint_x += dist,
            West(dist) => waypoint_x -= dist,
            Left(angle) => {
                for _ in 0..(angle / 90) {
                    // Rotation matrix: [[0, -1], [1, 0]]
                    (waypoint_x, waypoint_y) = (-waypoint_y, waypoint_x);
                }
            }
            Right(angle) => {
                for _ in 0..(angle / 90) {
                    // Rotation matrix: [[0, 1], [-1, 0]]
                    (waypoint_x, waypoint_y) = (waypoint_y, -waypoint_x);
                }
            }
            Forward(dist) => {
                ship_x += waypoint_x * dist;
                ship_y += waypoint_y * dist;
            }
        }
    }
    ship_x.abs() + ship_y.abs()
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
