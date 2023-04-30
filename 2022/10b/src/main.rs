use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }
        let mut parser = PullParser::new(s);
        parser.skip("addx ")?;
        let arg = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Instruction::Addx(arg))
    }
}

fn solve(input: Input) -> String {
    let mut i = 0;
    let mut x: i32 = 1;
    let mut screen = Grid::<char>::from_fn(GridBounds::new(6, 40), |_: Coords| '.');
    for inst in input.parse_lines::<Instruction>() {
        let cursor_y = i / 40;
        let cursor_x = i % 40;
        i += 1;
        if x.abs_diff(cursor_x) <= 1 {
            screen[(
                usize::try_from(cursor_y).unwrap(),
                usize::try_from(cursor_x).unwrap(),
            )] = '#';
        }
        if let Instruction::Addx(arg) = inst {
            let cursor_y = i / 40;
            let cursor_x = i % 40;
            i += 1;
            if x.abs_diff(cursor_x) <= 1 {
                screen[(
                    usize::try_from(cursor_y).unwrap(),
                    usize::try_from(cursor_x).unwrap(),
                )] = '#';
            }
            x += arg;
        }
    }
    screen.to_string()
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
            "addx 15\n",
            "addx -11\n",
            "addx 6\n",
            "addx -3\n",
            "addx 5\n",
            "addx -1\n",
            "addx -8\n",
            "addx 13\n",
            "addx 4\n",
            "noop\n",
            "addx -1\n",
            "addx 5\n",
            "addx -1\n",
            "addx 5\n",
            "addx -1\n",
            "addx 5\n",
            "addx -1\n",
            "addx 5\n",
            "addx -1\n",
            "addx -35\n",
            "addx 1\n",
            "addx 24\n",
            "addx -19\n",
            "addx 1\n",
            "addx 16\n",
            "addx -11\n",
            "noop\n",
            "noop\n",
            "addx 21\n",
            "addx -15\n",
            "noop\n",
            "noop\n",
            "addx -3\n",
            "addx 9\n",
            "addx 1\n",
            "addx -3\n",
            "addx 8\n",
            "addx 1\n",
            "addx 5\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx -36\n",
            "noop\n",
            "addx 1\n",
            "addx 7\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx 2\n",
            "addx 6\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx 1\n",
            "noop\n",
            "noop\n",
            "addx 7\n",
            "addx 1\n",
            "noop\n",
            "addx -13\n",
            "addx 13\n",
            "addx 7\n",
            "noop\n",
            "addx 1\n",
            "addx -33\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx 2\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx 8\n",
            "noop\n",
            "addx -1\n",
            "addx 2\n",
            "addx 1\n",
            "noop\n",
            "addx 17\n",
            "addx -9\n",
            "addx 1\n",
            "addx 1\n",
            "addx -3\n",
            "addx 11\n",
            "noop\n",
            "noop\n",
            "addx 1\n",
            "noop\n",
            "addx 1\n",
            "noop\n",
            "noop\n",
            "addx -13\n",
            "addx -19\n",
            "addx 1\n",
            "addx 3\n",
            "addx 26\n",
            "addx -30\n",
            "addx 12\n",
            "addx -1\n",
            "addx 3\n",
            "addx 1\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx -9\n",
            "addx 18\n",
            "addx 1\n",
            "addx 2\n",
            "noop\n",
            "noop\n",
            "addx 9\n",
            "noop\n",
            "noop\n",
            "noop\n",
            "addx -1\n",
            "addx 2\n",
            "addx -37\n",
            "addx 1\n",
            "addx 3\n",
            "noop\n",
            "addx 15\n",
            "addx -21\n",
            "addx 22\n",
            "addx -6\n",
            "addx 1\n",
            "noop\n",
            "addx 2\n",
            "addx 1\n",
            "noop\n",
            "addx -10\n",
            "noop\n",
            "noop\n",
            "addx 20\n",
            "addx 1\n",
            "addx 2\n",
            "addx 2\n",
            "addx -6\n",
            "addx -11\n",
            "noop\n",
            "noop\n",
            "noop\n",
        ));
        assert_eq!(
            solve(input),
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....",
            )
        );
    }
}
