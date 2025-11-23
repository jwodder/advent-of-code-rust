use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
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
            "nop" => Ok(Nop(arg)),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

fn solve(input: Input) -> i32 {
    use Operation::*;
    let program = input.parse_lines::<Operation>().collect::<Vec<_>>();
    for i in 0..program.len() {
        match program[i] {
            Jmp(arg) => {
                let mut newprogram = program.clone();
                newprogram[i] = Nop(arg);
                if let Some(acc) = run(newprogram) {
                    return acc;
                }
            }
            Nop(arg) => {
                let mut newprogram = program.clone();
                newprogram[i] = Jmp(arg);
                if let Some(acc) = run(newprogram) {
                    return acc;
                }
            }
            _ => (),
        }
    }
    panic!("No solution found");
}

fn run(program: Vec<Operation>) -> Option<i32> {
    use Operation::*;
    let mut acc = 0;
    let mut i = 0;
    let mut seen = HashSet::new();
    loop {
        let i_usize = usize::try_from(i).ok()?;
        if !seen.insert(i) {
            return None;
        } else if i_usize == program.len() {
            return Some(acc);
        }
        match program.get(i_usize)? {
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
        assert_eq!(solve(input), 8);
    }
}
