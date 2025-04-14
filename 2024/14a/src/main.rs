use adventutil::counter::Counter;
use adventutil::gridgeom::{Point, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    p: Point,
    v: Vector,
}

impl Robot {
    fn zoom(&self, floor: Floor, time: i32) -> Point {
        let p = self.p + self.v * time;
        Point {
            x: p.x.rem_euclid(floor.width),
            y: p.y.rem_euclid(floor.height),
        }
    }
}

impl std::str::FromStr for Robot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Robot, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("p=")?;
        let p_x = parser.parse_to::<i32, _>(',')?;
        let p_y = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("v=")?;
        let v_x = parser.parse_to::<i32, _>(',')?;
        let v_y = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Robot {
            p: Point { x: p_x, y: p_y },
            v: Vector { x: v_x, y: v_y },
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Floor {
    width: i32,
    height: i32,
}

impl Floor {
    fn quadrant(&self, p: Point) -> Option<Quadrant> {
        let half_width = self.width / 2;
        let is_west = if p.x < half_width {
            true
        } else if (self.width - 1 - p.x) < half_width {
            false
        } else {
            return None;
        };
        let half_height = self.height / 2;
        let is_north = if p.y < half_height {
            true
        } else if (self.height - 1 - p.y) < half_height {
            false
        } else {
            return None;
        };
        match (is_north, is_west) {
            (true, true) => Some(Quadrant::NW),
            (true, false) => Some(Quadrant::NE),
            (false, true) => Some(Quadrant::SW),
            (false, false) => Some(Quadrant::SE),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Quadrant {
    NW,
    NE,
    SW,
    SE,
}

fn solve(input: Input, floor: Floor) -> u64 {
    input
        .parse_lines::<Robot>()
        .filter_map(|r| floor.quadrant(r.zoom(floor, 100)))
        .collect::<Counter<Quadrant>>()
        .into_values()
        .product()
}

fn main() {
    println!(
        "{}",
        solve(
            Input::from_env(),
            Floor {
                width: 101,
                height: 103
            }
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "p=0,4 v=3,-3\n",
            "p=6,3 v=-1,-3\n",
            "p=10,3 v=-1,2\n",
            "p=2,0 v=2,-1\n",
            "p=0,0 v=1,3\n",
            "p=3,0 v=-2,-2\n",
            "p=7,6 v=-1,-3\n",
            "p=3,0 v=-1,-2\n",
            "p=9,3 v=2,3\n",
            "p=7,3 v=-1,2\n",
            "p=2,4 v=2,-3\n",
            "p=9,5 v=-3,-3\n",
        ));
        assert_eq!(
            solve(
                input,
                Floor {
                    width: 11,
                    height: 7
                }
            ),
            12
        );
    }
}
