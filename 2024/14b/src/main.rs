// The problem statement doesn't tell us what the Christmas tree looks like, so
// I'm going to assume it's like the design used for the 2015 calendar, e.g.:
//
// .......
// ...*...
// ..***..
// .*****.
// .......

use adventutil::gridgeom::{Point, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;

const FLOOR_WIDTH: i32 = 101;
const FLOOR_HEIGHT: i32 = 103;

// A guess
const TREE_HEIGHT: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    p: Point,
    v: Vector,
}

impl Robot {
    fn tick(&self) -> Robot {
        let Point { x, y } = self.p + self.v;
        let p = Point {
            x: x.rem_euclid(FLOOR_WIDTH),
            y: y.rem_euclid(FLOOR_HEIGHT),
        };
        Robot { p, v: self.v }
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

fn solve(input: Input) -> usize {
    let robots0 = input.parse_lines::<Robot>().collect::<Vec<_>>();
    std::iter::successors(Some(robots0), |robots| {
        Some(robots.iter().map(Robot::tick).collect())
    })
    .position(|robots| {
        let positions = robots.into_iter().map(|r| r.p).collect::<HashSet<_>>();
        positions
            .iter()
            .any(|&p| mktree(p).is_some_and(|tree| tree.is_subset(&positions)))
    })
    .unwrap()
}

fn mktree(star: Point) -> Option<HashSet<Point>> {
    // Does not include the "star":
    let mut tree = HashSet::new();
    let mut start_x = star.x;
    let mut end_x = star.x;
    let mut y = star.y;
    for _ in 1..TREE_HEIGHT {
        start_x = start_x.checked_sub(1)?;
        end_x += 1;
        if end_x >= FLOOR_WIDTH {
            return None;
        }
        y += 1;
        if y >= FLOOR_HEIGHT {
            return None;
        }
        tree.extend((start_x..=end_x).map(|x| Point { x, y }));
    }
    Some(tree)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
