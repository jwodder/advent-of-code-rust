use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Snd(Operand),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "snd" => {
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Snd(arg))
            }
            "set" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Set(reg, arg))
            }
            "add" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Add(reg, arg))
            }
            "mul" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Mul(reg, arg))
            }
            "mod" => {
                let reg = parser.parse_to::<char, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Mod(reg, arg))
            }
            "rcv" => {
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Rcv(arg))
            }
            "jgz" => {
                let reg = parser.parse_to::<Operand, _>(Token::Whitespace)?;
                let arg = parser.parse_to::<Operand, _>(Token::Eof)?;
                Ok(Instruction::Jgz(reg, arg))
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
    let mut last_sound = None;
    let mut i = 0;
    loop {
        match instructions[i] {
            Instruction::Snd(arg) => last_sound = Some(registers.get(arg)),
            Instruction::Set(reg, arg) => {
                let value = registers.get(arg);
                registers.set(reg, value);
            }
            Instruction::Add(reg, arg) => {
                let before = registers.get(Operand::Register(reg));
                let value = registers.get(arg);
                registers.set(reg, before + value);
            }
            Instruction::Mul(reg, arg) => {
                let before = registers.get(Operand::Register(reg));
                let value = registers.get(arg);
                registers.set(reg, before * value);
            }
            Instruction::Mod(reg, arg) => {
                let before = registers.get(Operand::Register(reg));
                let value = registers.get(arg);
                registers.set(reg, before % value);
            }
            Instruction::Rcv(arg) => {
                if registers.get(arg) != 0 {
                    return last_sound.unwrap();
                }
            }
            Instruction::Jgz(flag, offset) => {
                let flag = registers.get(flag);
                let offset = registers.get(offset);
                if flag > 0 {
                    i = usize::try_from(i64::try_from(i).unwrap() + offset).unwrap();
                    continue;
                }
            }
        }
        i += 1;
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
            "set a 1\n",
            "add a 2\n",
            "mul a a\n",
            "mod a 5\n",
            "snd a\n",
            "set a 0\n",
            "rcv a\n",
            "jgz a -1\n",
            "set a 1\n",
            "jgz a -2\n",
        ));
        assert_eq!(solve(input), 4);
    }
}
