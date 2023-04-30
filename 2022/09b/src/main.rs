use adventutil::gridgeom::{points_added, Point, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
use std::str::FromStr;

enum Motion {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Motion {
    fn into_vector(self) -> Vector {
        use Motion::*;
        match self {
            Up(dist) => Vector::NORTH * dist,
            Down(dist) => Vector::SOUTH * dist,
            Left(dist) => Vector::WEST * dist,
            Right(dist) => Vector::EAST * dist,
        }
    }
}

impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Motion, ParseError> {
        use Motion::*;
        let mut parser = PullParser::new(s);
        let dir = parser.scan_to(Token::Whitespace)?;
        let dist = parser.parse_to::<i32, _>(Token::Eof)?;
        match dir {
            "U" => Ok(Up(dist)),
            "D" => Ok(Down(dist)),
            "L" => Ok(Left(dist)),
            "R" => Ok(Right(dist)),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn solve(input: Input) -> usize {
    let mut positions = vec![Point::ORIGIN; 10];
    let mut visited = HashSet::from([Point::ORIGIN]);
    for motion in input.parse_lines::<Motion>() {
        for p in points_added(positions[0], motion.into_vector()).unwrap() {
            positions[0] = p;
            for i in 0..9 {
                let body = positions[i] - positions[i + 1];
                let tail = &mut positions[i + 1];
                if body.x.abs() > 1 || body.y.abs() > 1 {
                    tail.x += body.x.signum();
                    tail.y += body.y.signum();
                }
            }
            visited.insert(positions[9]);
        }
    }
    visited.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn test_example2() {
        let input = Input::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n");
        assert_eq!(solve(input), 36);
    }
}
