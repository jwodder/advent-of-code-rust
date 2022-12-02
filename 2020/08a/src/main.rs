use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
use std::str::FromStr;

enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, ParseError> {
        use Operation::*;
        let mut parser = PullParser::new(s);
        let op = parser.scan_to(Token::Whitespace)?;
        let arg = parser.parse_to::<i32, _>(Token::Eof)?;
        match op {
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            "nop" => Ok(Nop(arg)),
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
            Nop(_) => {
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
mod test {
    use super::*;

    #[test]
    fn test_example1() {
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
