use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Set(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Jnz(Operand, Operand),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "set" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Set(reg, arg))
            }
            "sub" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Sub(reg, arg))
            }
            "mul" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Mul(reg, arg))
            }
            "jnz" => {
                let reg = parser.parse_to::<Operand, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Jnz(reg, arg))
            }
            s => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operand {
    Register(char),
    Int(i64),
}

impl std::str::FromStr for Operand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operand, ParseError> {
        let mut chars = s.chars();
        let c1 = chars.next();
        let c2 = chars.next();
        match (c1, c2) {
            (Some(c), None) if c.is_ascii_lowercase() => Ok(Operand::Register(c)),
            _ => Ok(Operand::Int(s.parse::<i64>()?)),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Registers(HashMap<char, i64>);

impl Registers {
    fn get(&self, arg: Operand) -> i64 {
        match arg {
            Operand::Register(r) => self.0.get(&r).copied().unwrap_or(0),
            Operand::Int(i) => i,
        }
    }

    fn set(&mut self, reg: char, value: i64) {
        self.0.insert(reg, value);
    }
}

fn solve(input: Input) -> i64 {
    let instructions = input.parse_lines::<Instruction>().collect::<Vec<_>>();
    let mut registers = Registers::default();
    let mut muls = 0;
    let mut i = 0;
    while i < instructions.len() {
        match instructions[i] {
            Instruction::Set(reg, arg) => {
                let value = registers.get(arg);
                registers.set(reg, value);
            }
            Instruction::Sub(reg, arg) => {
                let before = registers.get(Operand::Register(reg));
                let value = registers.get(arg);
                registers.set(reg, before - value);
            }
            Instruction::Mul(reg, arg) => {
                let before = registers.get(Operand::Register(reg));
                let value = registers.get(arg);
                registers.set(reg, before * value);
                muls += 1;
            }
            Instruction::Jnz(flag, offset) => {
                let flag = registers.get(flag);
                let offset = registers.get(offset);
                if flag != 0 {
                    i = usize::try_from(i64::try_from(i).unwrap() + offset).unwrap();
                    continue;
                }
            }
        }
        i += 1;
    }
    muls
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
