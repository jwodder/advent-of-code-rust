use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl std::str::FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Register, ParseError> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "hlf" => {
                let register = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Hlf(register))
            }
            "tpl" => {
                let register = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Tpl(register))
            }
            "inc" => {
                let register = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Inc(register))
            }
            "jmp" => {
                let offset = parser.parse_to::<i32, _>(Token::Eof)?;
                Ok(Instruction::Jmp(offset))
            }
            "jie" => {
                let register = parser.parse_to::<Register, _>(',')?;
                parser.skip(Token::Whitespace)?;
                let offset = parser.parse_to::<i32, _>(Token::Eof)?;
                Ok(Instruction::Jie(register, offset))
            }
            "jio" => {
                let register = parser.parse_to::<Register, _>(',')?;
                parser.skip(Token::Whitespace)?;
                let offset = parser.parse_to::<i32, _>(Token::Eof)?;
                Ok(Instruction::Jio(register, offset))
            }
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

struct State {
    a: usize,
    b: usize,
}

impl State {
    fn new() -> State {
        State { a: 1, b: 0 }
    }

    fn get(&self, r: Register) -> usize {
        match r {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set(&mut self, r: Register, value: usize) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }
}

fn run<I: IntoIterator<Item = Instruction>>(iter: I) -> State {
    let instructions = iter.into_iter().collect::<Vec<_>>();
    let len = i32::try_from(instructions.len()).unwrap();
    let mut state = State::new();
    let mut i = 0;
    while (0..len).contains(&i) {
        match instructions[usize::try_from(i).unwrap()] {
            Instruction::Hlf(r) => {
                let value = state.get(r) / 2;
                state.set(r, value);
                i += 1;
            }
            Instruction::Tpl(r) => {
                let value = state.get(r) * 3;
                state.set(r, value);
                i += 1;
            }
            Instruction::Inc(r) => {
                let value = state.get(r) + 1;
                state.set(r, value);
                i += 1;
            }
            Instruction::Jmp(offset) => {
                i += offset;
            }
            Instruction::Jie(r, offset) => {
                if state.get(r).is_multiple_of(2) {
                    i += offset;
                } else {
                    i += 1;
                }
            }
            Instruction::Jio(r, offset) => {
                if state.get(r) == 1 {
                    i += offset;
                } else {
                    i += 1;
                }
            }
        }
    }
    state
}

fn solve(input: Input) -> usize {
    run(input.parse_lines::<Instruction>()).b
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
