use adventutil::Input;
use adventutil::gridgeom::{Point, PointBounds, points_added};
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct RockPath(Vec<Point>);

impl RockPath {
    fn points(self) -> Vec<Point> {
        let mut points = vec![self.0[0]];
        for (p1, p2) in self.0.into_iter().tuple_windows() {
            points.extend(points_added(p1, p2 - p1).unwrap());
        }
        points
    }
}

impl std::str::FromStr for RockPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RockPath, ParseError> {
        let joints = PullParser::new(s).delimited(" -> ", |t| {
            let mut parser = PullParser::new(t);
            let x = parser.parse_to::<i32, _>(',')?;
            let y = parser.parse_to::<i32, _>(Token::Eof)?;
            Ok(Point { x, y })
        })?;
        Ok(RockPath(joints))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

fn solve(input: Input) -> usize {
    let points = input
        .parse_lines::<RockPath>()
        .flat_map(RockPath::points)
        .collect::<Vec<_>>();
    let pbounds = PointBounds::for_points(points.iter().copied())
        .unwrap()
        .with_point(Point::ORIGIN);
    let ground_y = pbounds.max_y + 2;
    let mut world = points
        .into_iter()
        .map(|p| (p, Tile::Rock))
        .collect::<HashMap<_, _>>();
    for i in 1.. {
        let mut p = Point { x: 500, y: 0 };
        loop {
            let mut moving = false;
            for (xdiff, ydiff) in [(0, 1), (-1, 1), (1, 1)] {
                let p2 = Point {
                    x: p.x + xdiff,
                    y: p.y + ydiff,
                };
                let tile = if p2.y == ground_y {
                    Tile::Rock
                } else {
                    *world.get(&p2).unwrap_or(&Tile::Air)
                };
                if tile == Tile::Air {
                    p = p2;
                    moving = true;
                    break;
                }
            }
            if !moving {
                world.insert(p, Tile::Sand);
                if p == (Point { x: 500, y: 0 }) {
                    return i;
                }
                break;
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n");
        assert_eq!(solve(input), 93);
    }
}
