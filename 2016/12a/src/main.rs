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
    Copy { src: Source, dest: Register },
    Inc(Register),
    Dec(Register),
    Jnz { check: Source, delta: i32 },
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "cpy" => {
                let src = parser.parse_to::<Source, _>(Token::Whitespace)?;
                let dest = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Copy { src, dest })
            }
            "inc" => {
                let reg = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Inc(reg))
            }
            "dec" => {
                let reg = parser.parse_to::<Register, _>(Token::Eof)?;
                Ok(Instruction::Dec(reg))
            }
            "jnz" => {
                let check = parser.parse_to::<Source, _>(Token::Whitespace)?;
                let delta = parser.parse_to::<i32, _>(Token::Eof)?;
                Ok(Instruction::Jnz { check, delta })
            }
            s => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

fn solve(input: Input) -> i32 {
    let instructions = input.parse_lines::<Instruction>().collect::<Vec<_>>();
    let mut registers = std::collections::HashMap::from([
        (Register::A, 0i32),
        (Register::B, 0i32),
        (Register::C, 0i32),
        (Register::D, 0i32),
    ]);
    let mut i = 0;
    while i < instructions.len() {
        match instructions[i] {
            Instruction::Copy { src, dest } => {
                let value = match src {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                registers.insert(dest, value);
                i += 1;
            }
            Instruction::Inc(reg) => {
                *registers.get_mut(&reg).unwrap() += 1;
                i += 1;
            }
            Instruction::Dec(reg) => {
                *registers.get_mut(&reg).unwrap() -= 1;
                i += 1;
            }
            Instruction::Jnz { check, delta } => {
                let value = match check {
                    Source::Register(reg) => registers[&reg],
                    Source::Int(j) => j,
                };
                if value != 0 {
                    i = usize::try_from(i32::try_from(i).unwrap() + delta).unwrap();
                } else {
                    i += 1;
                }
            }
        }
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
            "cpy 41 a\n",
            "inc a\n",
            "inc a\n",
            "dec a\n",
            "jnz a 2\n",
            "dec a\n",
        ));
        assert_eq!(solve(input), 42);
    }
}
