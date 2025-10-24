use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Snd(Operand),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
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
                let arg = parser.parse_to::<char, _>(Token::Eof)?;
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program<'a> {
    registers: Registers,
    i: usize,
    instructions: &'a Vec<Instruction>,
    state: State,
}

impl<'a> Program<'a> {
    fn new(pid: i64, instructions: &'a Vec<Instruction>) -> Self {
        let mut registers = Registers::default();
        registers.set('p', pid);
        Program {
            registers,
            i: 0,
            instructions,
            state: State::Running,
        }
    }

    fn run(&mut self) -> Option<i64> {
        while self.i < self.instructions.len() {
            match self.instructions[self.i] {
                Instruction::Snd(arg) => {
                    self.i += 1;
                    return Some(self.registers.get(arg));
                }
                Instruction::Set(reg, arg) => {
                    let value = self.registers.get(arg);
                    self.registers.set(reg, value);
                }
                Instruction::Add(reg, arg) => {
                    let before = self.registers.get(Operand::Register(reg));
                    let value = self.registers.get(arg);
                    self.registers.set(reg, before + value);
                }
                Instruction::Mul(reg, arg) => {
                    let before = self.registers.get(Operand::Register(reg));
                    let value = self.registers.get(arg);
                    self.registers.set(reg, before * value);
                }
                Instruction::Mod(reg, arg) => {
                    let before = self.registers.get(Operand::Register(reg));
                    let value = self.registers.get(arg);
                    self.registers.set(reg, before % value);
                }
                Instruction::Rcv(_) => {
                    self.state = State::Awaiting;
                    return None;
                }
                Instruction::Jgz(flag, offset) => {
                    let flag = self.registers.get(flag);
                    let offset = self.registers.get(offset);
                    if flag > 0 {
                        self.i = usize::try_from(i64::try_from(self.i).unwrap() + offset).unwrap();
                        continue;
                    }
                }
            }
            self.i += 1;
        }
        self.state = State::Terminated;
        None
    }

    fn send_in(&mut self, value: i64) {
        let Instruction::Rcv(reg) = self.instructions[self.i] else {
            panic!("Program sent instruction when it wasn't receiving");
        };
        self.registers.set(reg, value);
        self.i += 1;
        self.state = State::Running;
    }

    fn is_active(&self) -> bool {
        self.state != State::Terminated
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Running,
    Awaiting,
    Terminated,
}

fn solve(input: Input) -> usize {
    let instructions = input.parse_lines::<Instruction>().collect::<Vec<_>>();
    let mut p0 = Program::new(0, &instructions);
    let mut p1 = Program::new(1, &instructions);
    let mut p0_to_p1 = VecDeque::new();
    let mut p1_to_p0 = VecDeque::new();
    let mut sent_by_p1 = 0;
    while p0.is_active() && p1.is_active() {
        match p0.state {
            State::Running => {
                if let Some(value) = p0.run() {
                    p0_to_p1.push_back(value);
                }
            }
            State::Awaiting => {
                if let Some(value) = p1_to_p0.pop_front() {
                    p0.send_in(value);
                } else if !(p1.state == State::Running
                    || (p1.state == State::Awaiting && !p0_to_p1.is_empty()))
                {
                    // Deadlock!
                    break;
                }
            }
            State::Terminated => (),
        }
        match p1.state {
            State::Running => {
                if let Some(value) = p1.run() {
                    sent_by_p1 += 1;
                    p1_to_p0.push_back(value);
                }
            }
            State::Awaiting => {
                if let Some(value) = p0_to_p1.pop_front() {
                    p1.send_in(value);
                } else if !(p0.state == State::Running
                    || (p0.state == State::Awaiting && !p1_to_p0.is_empty()))
                {
                    // Deadlock!
                    break;
                }
            }
            State::Terminated => (),
        }
    }
    sent_by_p1
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d\n");
        assert_eq!(solve(input), 3);
    }
}
