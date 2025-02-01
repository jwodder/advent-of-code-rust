use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Register {
    A,
    B,
    C,
    D,
}

impl std::str::FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Register, ParseError> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Source {
    Register(Register),
    Int(i32),
}

impl std::str::FromStr for Source {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Source, ParseError> {
        if let Ok(i) = s.parse::<i32>() {
            Ok(Source::Int(i))
        } else {
            let reg = s.parse::<Register>()?;
            Ok(Source::Register(reg))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Copy { src: Source, dest: Source },
    Inc(Source),
    Dec(Source),
    Jnz { check: Source, delta: Source },
    Tgl(Source),
}

impl Instruction {
    fn toggle(&mut self) {
        match self {
            Instruction::Copy { src, dest } => {
                *self = Instruction::Jnz {
                    check: *src,
                    delta: *dest,
                }
            }
            Instruction::Inc(reg) => *self = Instruction::Dec(*reg),
            Instruction::Dec(reg) => *self = Instruction::Inc(*reg),
            Instruction::Jnz { check, delta } => {
                *self = Instruction::Copy {
                    src: *check,
                    dest: *delta,
                }
            }
            Instruction::Tgl(src) => *self = Instruction::Inc(*src),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "cpy" => {
                let src = parser.parse_to::<Source, _>(Token::Whitespace)?;
                let dest = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Copy {
                    src,
                    dest: Source::Register(dest),
                })
            }
            "inc" => {
                let reg = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Inc(Source::Register(reg)))
            }
            "dec" => {
                let reg = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Dec(Source::Register(reg)))
            }
            "jnz" => {
                let check = parser.parse_to::<Source, _>(Token::Whitespace)?;
                let delta = parser.parse_to::<Source, _>(Token::Eof)?;
                Ok(Instruction::Jnz { check, delta })
            }
            "tgl" => {
                let source = parser.parse_to::<Source, _>(Token::Eof)?;
                Ok(Instruction::Tgl(source))
            }
            s => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

fn solve(input: Input) -> i32 {
    let mut instructions = input.parse_lines::<Instruction>().collect::<Vec<_>>();
    let mut registers = std::collections::HashMap::from([
        (Register::A, 7i32),
        (Register::B, 0i32),
        (Register::C, 0i32),
        (Register::D, 0i32),
    ]);
    let mut i = 0;
    while i < instructions.len() {
        match instructions[i] {
            Instruction::Copy {
                src,
                dest: Source::Register(dest),
            } => {
                let value = match src {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                registers.insert(dest, value);
            }
            Instruction::Copy {
                dest: Source::Int(_),
                ..
            } => (),
            Instruction::Inc(Source::Register(reg)) => {
                *registers.get_mut(&reg).unwrap() += 1;
            }
            Instruction::Inc(Source::Int(_)) => (),
            Instruction::Dec(Source::Register(reg)) => {
                *registers.get_mut(&reg).unwrap() -= 1;
            }
            Instruction::Dec(Source::Int(_)) => (),
            Instruction::Jnz { check, delta } => {
                let value = match check {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                let delta = match delta {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                if value != 0 {
                    i = usize::try_from(i32::try_from(i).unwrap() + delta).unwrap();
                    continue;
                }
            }
            Instruction::Tgl(src) => {
                let delta = match src {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                let i = usize::try_from(i32::try_from(i).unwrap() + delta).unwrap();
                if let Some(instr) = instructions.get_mut(i) {
                    instr.toggle();
                }
            }
        }
        i += 1;
    }
    registers[&Register::A]
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "cpy 2 a\n",
            "tgl a\n",
            "tgl a\n",
            "tgl a\n",
            "cpy 1 a\n",
            "dec a\n",
            "dec a\n",
        ));
        assert_eq!(solve(input), 3);
    }
}
