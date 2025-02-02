use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{components, Input};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn is_adjacent(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x)
            + self.y.abs_diff(other.y)
            + self.z.abs_diff(other.z)
            + self.t.abs_diff(other.t)
            <= 3
    }
}

impl std::str::FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Point, ParseError> {
        let mut parser = PullParser::new(s);
        let x = parser.parse_to::<i32, _>(',')?;
        let y = parser.parse_to::<i32, _>(',')?;
        let z = parser.parse_to::<i32, _>(',')?;
        let t = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Point { x, y, z, t })
    }
}

fn solve(input: Input) -> usize {
    let points = input.parse_lines::<Point>().collect::<Vec<_>>();
    components(points.clone(), |p| {
        points.iter().filter(move |&q| p.is_adjacent(q)).copied()
    })
    .len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "0,0,0,0\n",
            "3,0,0,0\n",
            "0,3,0,0\n",
            "0,0,3,0\n",
            "0,0,0,3\n",
            "0,0,0,6\n",
            "9,0,0,0\n",
            "12,0,0,0\n",
        ));
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "-1,2,2,0\n",
            "0,0,2,-2\n",
            "0,0,0,-2\n",
            "-1,2,0,0\n",
            "-2,-2,-2,2\n",
            "3,0,2,-1\n",
            "-1,3,2,2\n",
            "-1,0,-1,0\n",
            "0,2,1,-2\n",
            "3,0,0,0\n",
        ));
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_example3() {
        let input = Input::from(concat!(
            "1,-1,0,1\n",
            "2,0,-1,0\n",
            "3,2,-1,0\n",
            "0,0,3,1\n",
            "0,0,-1,-1\n",
            "2,3,-2,0\n",
            "-2,2,0,0\n",
            "2,-2,0,-1\n",
            "1,-1,0,-1\n",
            "3,2,0,2\n",
        ));
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_example4() {
        let input = Input::from(concat!(
            "1,-1,-1,-2\n",
            "-2,-2,0,1\n",
            "0,2,1,3\n",
            "-2,3,-2,1\n",
            "0,2,3,-2\n",
            "-1,-1,1,-2\n",
            "0,-2,-1,0\n",
            "-2,2,3,-1\n",
            "1,2,2,0\n",
            "-1,-2,0,-2\n",
        ));
        assert_eq!(solve(input), 8);
    }
}
