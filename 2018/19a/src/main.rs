use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use thiserror::Error;

type Int = usize;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Registers([Int; 6]);

impl Registers {
    fn get(&self, index: usize) -> Option<Int> {
        (0..6).contains(&index).then(|| self.0[index])
    }

    fn set(&mut self, index: usize, value: Int) -> bool {
        if (0..6).contains(&index) {
            self.0[index] = value;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Instruction {
    opcode: Opcode,
    a: Int,
    b: Int,
    c: Int,
}

impl Instruction {
    fn evaluate(self, regs: &mut Registers) -> bool {
        self.opcode.evaluate(regs, self.a, self.b, self.c)
    }
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        let opcode = parser.parse_to::<Opcode, _>(Token::Whitespace)?;
        let a = parser.parse_to::<Int, _>(Token::Whitespace)?;
        let b = parser.parse_to::<Int, _>(Token::Whitespace)?;
        let c = parser.parse_to::<Int, _>(Token::Eof)?;
        Ok(Instruction { opcode, a, b, c })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn evaluate(self, regs: &mut Registers, a: Int, b: Int, c: Int) -> bool {
        match self {
            Opcode::Addr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, reg_a + reg_b)
                } else {
                    false
                }
            }
            Opcode::Addi => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, reg_a + b)
                } else {
                    false
                }
            }
            Opcode::Mulr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, reg_a * reg_b)
                } else {
                    false
                }
            }
            Opcode::Muli => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, reg_a * b)
                } else {
                    false
                }
            }
            Opcode::Banr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, reg_a & reg_b)
                } else {
                    false
                }
            }
            Opcode::Bani => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, reg_a & b)
                } else {
                    false
                }
            }
            Opcode::Borr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, reg_a | reg_b)
                } else {
                    false
                }
            }
            Opcode::Bori => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, reg_a | b)
                } else {
                    false
                }
            }
            Opcode::Setr => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, reg_a)
                } else {
                    false
                }
            }
            Opcode::Seti => regs.set(c, a),
            Opcode::Gtir => {
                if let Some(reg_b) = regs.get(b) {
                    regs.set(c, Int::from(a > reg_b))
                } else {
                    false
                }
            }
            Opcode::Gtri => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, Int::from(reg_a > b))
                } else {
                    false
                }
            }
            Opcode::Gtrr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, Int::from(reg_a > reg_b))
                } else {
                    false
                }
            }
            Opcode::Eqir => {
                if let Some(reg_b) = regs.get(b) {
                    regs.set(c, Int::from(a == reg_b))
                } else {
                    false
                }
            }
            Opcode::Eqri => {
                if let Some(reg_a) = regs.get(a) {
                    regs.set(c, Int::from(reg_a == b))
                } else {
                    false
                }
            }
            Opcode::Eqrr => {
                if let Some(reg_a) = regs.get(a)
                    && let Some(reg_b) = regs.get(b)
                {
                    regs.set(c, Int::from(reg_a == reg_b))
                } else {
                    false
                }
            }
        }
    }
}

impl std::str::FromStr for Opcode {
    type Err = ParseOpcodeError;

    fn from_str(s: &str) -> Result<Opcode, ParseOpcodeError> {
        match s {
            "addr" => Ok(Opcode::Addr),
            "addi" => Ok(Opcode::Addi),
            "mulr" => Ok(Opcode::Mulr),
            "muli" => Ok(Opcode::Muli),
            "banr" => Ok(Opcode::Banr),
            "bani" => Ok(Opcode::Bani),
            "borr" => Ok(Opcode::Borr),
            "bori" => Ok(Opcode::Bori),
            "setr" => Ok(Opcode::Setr),
            "seti" => Ok(Opcode::Seti),
            "gtir" => Ok(Opcode::Gtir),
            "gtri" => Ok(Opcode::Gtri),
            "gtrr" => Ok(Opcode::Gtrr),
            "eqir" => Ok(Opcode::Eqir),
            "eqri" => Ok(Opcode::Eqri),
            "eqrr" => Ok(Opcode::Eqrr),
            _ => Err(ParseOpcodeError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid opcode: {0:?}")]
struct ParseOpcodeError(String);

fn solve(input: Input) -> Int {
    let mut lines = input.lines();
    let ipline = lines.next().unwrap();
    let ipreg = ipline.strip_prefix("#ip ").unwrap().parse::<Int>().unwrap();
    let instructions = lines
        .map(|ln| ln.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut regs = Registers::default();
    while i < instructions.len() {
        regs.set(ipreg, i);
        assert!(
            instructions[i].evaluate(&mut regs),
            "Instruction {i} failed"
        );
        i = regs.get(ipreg).unwrap() + 1;
    }
    regs.get(0).unwrap()
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
            "#ip 0\n",
            "seti 5 0 1\n",
            "seti 6 0 2\n",
            "addi 0 1 0\n",
            "addr 1 2 3\n",
            "setr 1 0 0\n",
            "seti 8 0 4\n",
            "seti 9 0 5\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
