use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn adjacent(self) -> Vec<Cube> {
        let mut adjs = Vec::new();
        for (xdiff, ydiff, zdiff) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            adjs.push(Cube {
                x: self.x + xdiff,
                y: self.y + ydiff,
                z: self.z + zdiff,
            });
        }
        adjs
    }
}

impl std::str::FromStr for Cube {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Cube, ParseError> {
        let mut parser = PullParser::new(s);
        let x = parser.parse_to::<i32, _>(',')?;
        let y = parser.parse_to::<i32, _>(',')?;
        let z = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Cube { x, y, z })
    }
}

fn solve(input: Input) -> usize {
    let cubes = input.parse_lines::<Cube>().collect::<HashSet<_>>();
    cubes
        .iter()
        .flat_map(|&cube| cube.adjacent())
        .filter(|cube| !cubes.contains(cube))
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("1,1,1\n2,1,1\n");
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "2,2,2\n", "1,2,2\n", "3,2,2\n", "2,1,2\n", "2,3,2\n", "2,2,1\n", "2,2,3\n", "2,2,4\n",
            "2,2,6\n", "1,2,5\n", "3,2,5\n", "2,1,5\n", "2,3,5\n",
        ));
        assert_eq!(solve(input), 64);
    }
}
