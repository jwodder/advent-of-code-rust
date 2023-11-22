#![allow(clippy::range_plus_one)]
use adventutil::gridgeom::Point;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::ops::Range;
use std::str::FromStr;

struct Sensor {
    pos: Point,
    beacon: Point,
}

impl Sensor {
    fn covered(&self) -> impl Iterator<Item = (i32, Range<i32>)> + '_ {
        let radius = (self.beacon - self.pos).taxicab_len();
        (-radius..=radius).map(move |dy| {
            let y = self.pos.y + dy;
            let dx = radius - dy.abs();
            (y, (self.pos.x - dx)..(self.pos.x + dx + 1))
        })
    }
}

impl FromStr for Sensor {
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct RangeSet(Vec<Range<i32>>);

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet(Vec::new())
    }

    fn add(&mut self, rng: Range<i32>) {
        let modified = match self.0.binary_search_by_key(&rng.start, |r| r.start) {
            Ok(i) if rng.end > self.0[i].end => {
                self.0[i] = rng;
                Some(i)
            }
            Ok(_) => None,
            Err(i) if i > 0 && rng.start <= self.0[i - 1].end => (rng.end > self.0[i - 1].end)
                .then(|| {
                    self.0[i - 1] = self.0[i - 1].start..rng.end;
                    i - 1
                }),
            Err(i) => {
                self.0.insert(i, rng);
                Some(i)
            }
        };
        if let Some(i) = modified {
            while i + 1 < self.0.len() && self.0[i].end >= self.0[i + 1].start {
                self.0[i].end = self.0[i + 1].end.max(self.0[i].end);
                self.0.remove(i + 1);
            }
        }
    }

    fn missing_in(&self, rng: Range<i32>) -> Vec<i32> {
        let mut prev = rng.start;
        let mut missing = Vec::new();
        for r in &self.0 {
            if r.end < rng.start {
                continue;
            }
            if r.start >= rng.end {
                break;
            }
            missing.extend(prev..r.start);
            prev = r.end;
        }
        missing
    }
}

fn solve(input: Input, size: i32) -> u64 {
    let mut rows = vec![RangeSet::new(); (size + 1).try_into().unwrap()];
    for sensor in input.parse_lines::<Sensor>() {
        for (y, rng) in sensor.covered() {
            if (0..=size).contains(&y) {
                rows[usize::try_from(y).unwrap()].add(rng);
            }
        }
    }
    for y in 0..=size {
        let missing = rows[usize::try_from(y).unwrap()].missing_in(0..(size + 1));
        if !missing.is_empty() {
            assert_eq!(missing.len(), 1);
            let x = missing[0];
            return u64::try_from(x).unwrap() * 4000000 + u64::try_from(y).unwrap();
        }
    }
    panic!("Beacon not found");
}

fn main() {
    println!("{}", solve(Input::from_env(), 4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
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
        assert_eq!(solve(input, 20), 56000011);
    }
}
