use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::str::FromStr;

enum Operation {
    Rect { width: usize, height: usize },
    RotateRow { y: usize, by: usize },
    RotateColumn { x: usize, by: usize },
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("rect ").is_ok() {
            let width = parser.parse_to::<usize, _>('x')?;
            let height = parser.parse_to::<usize, _>(Token::Eof)?;
            Ok(Operation::Rect { width, height })
        } else if parser.skip("rotate row y=").is_ok() {
            let y = parser.parse_to::<usize, _>(Token::Whitespace)?;
            parser.skip("by ")?;
            let by = parser.parse_to::<usize, _>(Token::Eof)?;
            Ok(Operation::RotateRow { y, by })
        } else {
            parser.skip("rotate column x=")?;
            let x = parser.parse_to::<usize, _>(Token::Whitespace)?;
            parser.skip("by ")?;
            let by = parser.parse_to::<usize, _>(Token::Eof)?;
            Ok(Operation::RotateColumn { x, by })
        }
    }
}

fn solve(input: Input) -> String {
    // TODO: Operate on adventutil's Grid type instead?
    let mut grid = vec![vec![false; 50]; 6];
    for op in input.parse_lines::<Operation>() {
        match op {
            Operation::Rect { width, height } => {
                for row in grid.iter_mut().take(height) {
                    for cell in row.iter_mut().take(width) {
                        *cell = true;
                    }
                }
            }
            Operation::RotateRow { y, by } => grid[y].rotate_right(by),
            Operation::RotateColumn { x, by } => {
                let mut col = grid.iter().map(|row| row[x]).collect::<Vec<_>>();
                col.rotate_right(by);
                for (c, row) in col.into_iter().zip(grid.iter_mut()) {
                    row[x] = c;
                }
            }
        }
    }
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .join("\n")
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
