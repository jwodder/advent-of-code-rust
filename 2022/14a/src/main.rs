use adventutil::Input;
use adventutil::grid::{Coords, Direction, Grid, GridBounds};
use adventutil::gridgeom::{Point, PointBounds, points_added};
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashSet;

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
        .collect::<HashSet<_>>();
    let pbounds = PointBounds::for_points(points.iter().copied())
        .unwrap()
        .with_point(Point::ORIGIN);
    let grbounds = GridBounds::new(
        usize::try_from(pbounds.height()).unwrap(),
        usize::try_from(pbounds.width()).unwrap(),
    );
    let mut grid = Grid::from_fn(grbounds, |c| {
        if points.contains(&pbounds.at_coords(c, false)) {
            Tile::Rock
        } else {
            Tile::Air
        }
    });
    for i in 0.. {
        let mut c = Coords::new(0, 500);
        loop {
            let mut moving = false;
            for d in [Direction::South, Direction::SouthWest, Direction::SouthEast] {
                if let Some(c2) = grbounds.move_in(c, d) {
                    match grid[c2] {
                        Tile::Air => {
                            c = c2;
                            moving = true;
                            break;
                        }
                        Tile::Rock | Tile::Sand => (),
                    }
                } else {
                    return i;
                }
            }
            if !moving {
                grid[c] = Tile::Sand;
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
        assert_eq!(solve(input), 24);
    }
}
