use adventutil::gridgeom::{points_added, Point, PointBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;
use std::str::FromStr;

struct RockPath(Vec<Point>);

impl RockPath {
    fn points(self) -> Vec<Point> {
        let mut points = vec![self.0[0]];
        for w in self.0.windows(2) {
            points.extend(points_added(w[0], w[1] - w[0]).unwrap());
        }
        points
    }
}

impl FromStr for RockPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RockPath, ParseError> {
        let joints = s
            .split(" -> ")
            .map(|t| -> Result<Point, ParseError> {
                let mut parser = PullParser::new(t);
                let x = parser.parse_to::<i32, _>(',')?;
                let y = parser.parse_to::<i32, _>(Token::Eof)?;
                Ok(Point { x, y })
            })
            .collect::<Result<Vec<_>, _>>()?;
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
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n");
        assert_eq!(solve(input), 93);
    }
}
