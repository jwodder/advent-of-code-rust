use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

type Int = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sample {
    before: Registers,
    opcode: Int,
    a: Int,
    b: Int,
    c: Int,
    after: Registers,
}

impl Sample {
    fn possibilities(&self) -> usize {
        Opcode::iter()
            .filter(|op| {
                let mut regs = self.before.clone();
                op.evaluate(&mut regs, self.a, self.b, self.c) && regs == self.after
            })
            .count()
    }
}

impl std::str::FromStr for Sample {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Sample, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Before:")?;
        parser.skip(Token::Whitespace)?;
        let before = parser.parse_to::<Registers, _>(Token::Newline)?;
        let opcode = parser.parse_to::<Int, _>(Token::Whitespace)?;
        let a = parser.parse_to::<Int, _>(Token::Whitespace)?;
        let b = parser.parse_to::<Int, _>(Token::Whitespace)?;
        let c = parser.parse_to::<Int, _>(Token::Newline)?;
        parser.skip("After:")?;
        parser.skip(Token::Whitespace)?;
        let after = parser.parse_to::<Registers, _>(Token::Eof)?;
        Ok(Sample {
            before,
            opcode,
            a,
            b,
            c,
            after,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Registers([Int; 4]);

impl Registers {
    fn get(&self, index: usize) -> Option<Int> {
        (0..4).contains(&index).then(|| self.0[index])
    }

    fn set(&mut self, index: usize, value: Int) -> bool {
        if (0..4).contains(&index) {
            self.0[index] = value;
            true
        } else {
            false
        }
    }
}

impl std::str::FromStr for Registers {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Registers, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip('[')?;
        let r0 = parser.parse_to::<Int, _>(", ")?;
        let r1 = parser.parse_to::<Int, _>(", ")?;
        let r2 = parser.parse_to::<Int, _>(", ")?;
        let r3 = parser.parse_to::<Int, _>(']')?;
        parser.eof()?;
        Ok(Registers([r0, r1, r2, r3]))
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
    fn iter() -> impl Iterator<Item = Opcode> {
        [
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ]
        .into_iter()
    }

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

fn solve(input: Input) -> usize {
    input
        .paragraphs()
        .filter(|para| para.starts_with("Before:"))
        .map(|para| para.parse::<Sample>().unwrap())
        .filter(|sm| sm.possibilities() >= 3)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let sample = concat!(
            "Before: [3, 2, 1, 1]\n",
            "9 2 1 2\n",
            "After:  [3, 2, 2, 1]",
        )
        .parse::<Sample>()
        .unwrap();
        assert_eq!(sample.possibilities(), 3);
    }
}
