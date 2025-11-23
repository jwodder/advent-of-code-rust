use adventutil::Input;
use adventutil::gridgeom::Point;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Sensor {
    pos: Point,
    beacon: Point,
}

impl Sensor {
    fn covered(&self, y: i32) -> Vec<Point> {
        let radius = (self.beacon - self.pos).taxicab_len();
        let delta_y = (y - self.pos.y).abs();
        let delta_x = radius - delta_y;
        if delta_x >= 0 {
            (0..=delta_x)
                .flat_map(|dx| {
                    [
                        Point {
                            x: self.pos.x + dx,
                            y,
                        },
                        Point {
                            x: self.pos.x - dx,
                            y,
                        },
                    ]
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl std::str::FromStr for Sensor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Sensor, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Sensor at x=")?;
        let pos_x = parser.parse_to::<i32, _>(',')?;
        parser.skip(" y=")?;
        let pos_y = parser.parse_to::<i32, _>(':')?;
        parser.skip(" closest beacon is at x=")?;
        let beacon_x = parser.parse_to::<i32, _>(',')?;
        parser.skip(" y=")?;
        let beacon_y = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Sensor {
            pos: Point { x: pos_x, y: pos_y },
            beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
        })
    }
}

fn solve(input: Input, y: i32) -> usize {
    let sensors = input.parse_lines::<Sensor>().collect::<Vec<_>>();
    let covered = sensors
        .iter()
        .flat_map(|s| s.covered(y))
        .collect::<HashSet<_>>();
    let beacons = sensors
        .iter()
        .filter_map(|s| (s.beacon.y == y).then_some(s.beacon))
        .collect::<HashSet<_>>();
    (&covered - &beacons).len()
}

fn main() {
    println!("{}", solve(Input::from_env(), 2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
        ));
        assert_eq!(solve(input, 10), 26);
    }
}
