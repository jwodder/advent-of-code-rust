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

struct Facing {
    xdir: i32,
    ydir: i32,
}

impl Facing {
    fn turn_left(self, angle: i32) -> Facing {
        let mut xdir = self.xdir;
        let mut ydir = self.ydir;
        for _ in 0..(angle / 90) {
            // Rotation matrix: [[0, -1], [1, 0]]
            (xdir, ydir) = (-ydir, xdir);
        }
        Facing { xdir, ydir }
    }

    fn turn_right(self, angle: i32) -> Facing {
        let mut xdir = self.xdir;
        let mut ydir = self.ydir;
        for _ in 0..(angle / 90) {
            // Rotation matrix: [[0, 1], [-1, 0]]
            (xdir, ydir) = (ydir, -xdir);
        }
        Facing { xdir, ydir }
    }

    fn displacement(&self, distance: i32) -> (i32, i32) {
        (self.xdir * distance, self.ydir * distance)
    }
}

fn solve(input: Input) -> i32 {
    let mut facing = Facing { xdir: 1, ydir: 0 }; // east
    let (mut x, mut y) = (0, 0);
    for instruction in input.parse_lines::<Instruction>() {
        use Instruction::*;
        match instruction {
            North(dist) => y += dist,
            South(dist) => y -= dist,
            East(dist) => x += dist,
            West(dist) => x -= dist,
            Left(angle) => facing = facing.turn_left(angle),
            Right(angle) => facing = facing.turn_right(angle),
            Forward(dist) => {
                let (xdiff, ydiff) = facing.displacement(dist);
                x += xdiff;
                y += ydiff;
            }
        }
    }
    x.abs() + y.abs()
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
        assert_eq!(solve(input), 25);
    }
}
