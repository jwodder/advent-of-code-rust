use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;

type Int = u32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Registers {
    a: Int,
    b: Int,
    c: Int,
}

impl Registers {
    fn combo(&self, operand: u8) -> Int {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

impl std::str::FromStr for Registers {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Registers, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Register A: ")?;
        let a = parser.parse_to::<Int, _>(Token::Newline)?;
        parser.skip("Register B: ")?;
        let b = parser.parse_to::<Int, _>(Token::Newline)?;
        parser.skip("Register C: ")?;
        let c = parser.parse_to::<Int, _>(Token::Eof)?;
        Ok(Registers { a, b, c })
    }
}

fn solve(input: Input) -> String {
    let (initial_reg, program) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let program = program
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut registers = initial_reg.parse::<Registers>().unwrap();
    let mut i = 0;
    let mut output = Vec::new();
    while i + 1 < program.len() {
        match program[i] {
            0 => {
                let num = registers.a;
                let denom = 1 << registers.combo(program[i + 1]);
                registers.a = num / denom;
            }
            1 => registers.b ^= Int::from(program[i + 1]),
            2 => {
                let value = registers.combo(program[i + 1]);
                registers.b = value % 8;
            }
            3 => {
                if registers.a != 0 {
                    i = usize::from(program[i + 1]);
                    continue;
                }
            }
            4 => registers.b ^= registers.c,
            5 => output.push(registers.combo(program[i + 1]) % 8),
            6 => {
                let num = registers.a;
                let denom = 1 << registers.combo(program[i + 1]);
                registers.b = num / denom;
            }
            7 => {
                let num = registers.a;
                let denom = 1 << registers.combo(program[i + 1]);
                registers.c = num / denom;
            }
            _ => unreachable!(),
        }
        i += 2;
    }
    output.into_iter().join(",")
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
            "Register A: 729\n",
            "Register B: 0\n",
            "Register C: 0\n",
            "\n",
            "Program: 0,1,5,4,3,0\n",
        ));
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
