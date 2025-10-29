use adventutil::gridgeom::{Point, Vector, points_added};
use adventutil::pullparser::ParseError;
use adventutil::{Input, parse_csv};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Direction, ParseError> {
        use Direction::*;
        let inst = s
            .get(0..1)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?;
        let arg = s
            .get(1..)
            .ok_or_else(|| ParseError::InvalidToken(s.into()))?
            .parse::<i32>()?;
        match inst {
            "U" => Ok(Up(arg)),
            "D" => Ok(Down(arg)),
            "L" => Ok(Left(arg)),
            "R" => Ok(Right(arg)),
            _ => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn wire_path(s: String) -> HashSet<Point> {
    let mut pos = Point::ORIGIN;
    let mut visited = HashSet::new();
    for d in parse_csv::<Direction>(&s) {
        let v = match d {
            Direction::Up(dist) => Vector::NORTH * dist,
            Direction::Down(dist) => Vector::SOUTH * dist,
            Direction::Left(dist) => Vector::WEST * dist,
            Direction::Right(dist) => Vector::EAST * dist,
        };
        visited.extend(points_added(pos, v).unwrap());
        pos += v;
    }
    visited
}

fn solve(input: Input) -> i32 {
    let (path1, path2) = input
        .lines()
        .map(wire_path)
        .collect_tuple()
        .expect("Input was not two lines long");
    path1
        .intersection(&path2)
        .map(|&p| (p - Point::ORIGIN).taxicab_len())
        .min()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("R8,U5,L5,D3\nU7,R6,D4,L4\n");
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
            "U62,R66,U55,R34,D71,R55,D58,R83\n",
        ));
        assert_eq!(solve(input), 159);
    }

    #[test]
    fn test_example3() {
        let input = Input::from(concat!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n",
        ));
        assert_eq!(solve(input), 135);
    }
}
