use adventutil::grid::{Coords, Direction, Grid, GridBounds};
use adventutil::gridgeom::{points_added, Point, PointBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
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
    let mut grid = Grid::from_fn(grbounds, |(y, x)| {
        let y = i32::try_from(y).unwrap();
        let x = i32::try_from(x).unwrap();
        if points.contains(&Point { x, y }) {
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
    fn test_example1() {
        let input = Input::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n");
        assert_eq!(solve(input), 24);
    }
}
