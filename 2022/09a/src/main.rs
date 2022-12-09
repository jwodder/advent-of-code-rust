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
    let mut head_pos = Point::ORIGIN;
    let mut tail_pos = Point::ORIGIN;
    let mut visited = HashSet::from([Point::ORIGIN]);
    for motion in input.parse_lines::<Motion>() {
        for p in points_added(head_pos, motion.into_vector()).unwrap() {
            head_pos = p;
            let body = head_pos - tail_pos;
            if body.x.abs() > 1 || body.y.abs() > 1 {
                tail_pos.x += body.x.signum();
                tail_pos.y += body.y.signum();
            }
            visited.insert(tail_pos);
        }
    }
    visited.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
        assert_eq!(solve(input), 13);
    }
}
