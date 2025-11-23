use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl std::str::FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, ParseError> {
        use Operation::*;
        let mut parser = PullParser::new(s);
        let op = parser.scan_to(Token::Whitespace)?;
        let arg = parser.parse_to::<i32, _>(Token::Eof)?;
        match op {
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            "nop" => Ok(Nop),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn solve(input: Input) -> i32 {
    let program = input.parse_lines::<Operation>().collect::<Vec<_>>();
    let mut acc = 0;
    let mut i = 0;
    let mut seen = HashSet::new();
    while seen.insert(i) {
        use Operation::*;
        match program[usize::try_from(i).unwrap()] {
            Acc(arg) => {
                acc += arg;
                i += 1;
            }
            Jmp(arg) => {
                i += arg;
            }
            Nop => {
                i += 1;
            }
        }
    }
    acc
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "nop +0\n",
            "acc +1\n",
            "jmp +4\n",
            "acc +3\n",
            "jmp -3\n",
            "acc -99\n",
            "acc +1\n",
            "jmp -4\n",
            "acc +6\n",
        ));
        assert_eq!(solve(input), 5);
    }
}
