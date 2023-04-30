use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Dot {
    x: usize,
    y: usize,
}

impl From<Dot> for Coords {
    fn from(dot: Dot) -> Coords {
        Coords { y: dot.y, x: dot.x }
    }
}

impl FromStr for Dot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Dot, ParseError> {
        let mut parser = PullParser::new(s);
        let x = parser.parse_to::<usize, _>(',')?;
        let y = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Dot { x, y })
    }
}

enum Fold {
    OverHorizontal(usize),
    OverVertical(usize),
}

impl FromStr for Fold {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Fold, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("fold along y=").is_ok() {
            let arg = parser.parse_to::<usize, _>(Token::Eof)?;
            Ok(Fold::OverHorizontal(arg))
        } else {
            parser.skip("fold along x=")?;
            let arg = parser.parse_to::<usize, _>(Token::Eof)?;
            Ok(Fold::OverVertical(arg))
        }
    }
}

fn solve(input: Input) -> String {
    let (dots, instructions) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let dots = dots
        .lines()
        .map(|s| Coords::from(s.parse::<Dot>().expect("Parse error")))
        .collect::<HashSet<_>>();
    let (mut max_y, mut max_x) = (0, 0);
    for &Coords { y, x } in &dots {
        if y > max_y {
            max_y = y;
        }
        if x > max_x {
            max_x = x;
        }
    }
    let mut bounds = GridBounds::new(max_y + 1, max_x + 1);
    let mut grid = Grid::<bool>::from_fn(bounds, |c| dots.contains(&c));
    for fold in instructions
        .lines()
        .map(|s| s.parse::<Fold>().expect("Parse error"))
    {
        match fold {
            Fold::OverHorizontal(axis) => {
                assert!((bounds.height / 2..bounds.height).contains(&axis));
                let newbounds = GridBounds::new(axis, bounds.width);
                let overlap = bounds.height - 1 - axis;
                grid = Grid::from_fn(newbounds, |c: Coords| {
                    if c.y < axis - overlap {
                        grid[c]
                    } else {
                        grid[c] || grid[(2 * axis - c.y, c.x)]
                    }
                });
                bounds = newbounds;
            }
            Fold::OverVertical(axis) => {
                assert!((bounds.width / 2..bounds.width).contains(&axis));
                let newbounds = GridBounds::new(bounds.height, axis);
                let overlap = bounds.width - 1 - axis;
                grid = Grid::from_fn(newbounds, |c: Coords| {
                    if c.x < axis - overlap {
                        grid[c]
                    } else {
                        grid[c] || grid[(c.y, 2 * axis - c.x)]
                    }
                });
                bounds = newbounds;
            }
        }
    }
    grid.map(|b| if b { '#' } else { '.' }).to_string()
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
            "6,10\n",
            "0,14\n",
            "9,10\n",
            "0,3\n",
            "10,4\n",
            "4,11\n",
            "6,0\n",
            "6,12\n",
            "4,1\n",
            "0,13\n",
            "10,12\n",
            "3,4\n",
            "3,0\n",
            "8,4\n",
            "1,10\n",
            "2,14\n",
            "8,10\n",
            "9,0\n",
            "\n",
            "fold along y=7\n",
            "fold along x=5\n",
        ));
        assert_eq!(
            solve(input),
            "#####\n#...#\n#...#\n#...#\n#####\n.....\n....."
        );
    }
}
