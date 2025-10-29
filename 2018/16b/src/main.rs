use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, HashSet};

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
    fn possibilities(&self) -> impl Iterator<Item = Opcode> {
        Opcode::iter().filter(|op| {
            let mut regs = self.before.clone();
            op.evaluate(&mut regs, self.a, self.b, self.c) && regs == self.after
        })
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

fn solve(input: Input) -> Int {
    let mut program = None;
    let mut num2ops: Vec<HashSet<Opcode>> = vec![HashSet::from_iter(Opcode::iter()); 16];
    for para in input.paragraphs() {
        if para.starts_with("Before:") {
            let sm = para.parse::<Sample>().unwrap();
            let possible_codes = sm.possibilities().collect::<HashSet<_>>();
            num2ops[sm.opcode].retain(|op| possible_codes.contains(op));
        } else {
            program = Some(para);
        }
    }
    let mut assignments = HashMap::new();
    let mut unassigned = num2ops.into_iter().enumerate().collect::<HashMap<_, _>>();
    while !unassigned.is_empty() {
        let Some((&i, ops)) = unassigned.iter().find(|(_, ops)| ops.len() == 1) else {
            panic!("No singleton possibility sets left!");
        };
        let op = ops.iter().next().copied().unwrap();
        unassigned.remove(&i);
        assignments.insert(i, op);
        for ops in unassigned.values_mut() {
            ops.remove(&op);
        }
    }
    let mut regs = Registers::default();
    for ln in program.unwrap().lines() {
        let [opcode, a, b, c] = *ln.split_whitespace().collect::<Vec<_>>() else {
            panic!("Invalid program line: {ln:?}");
        };
        let opcode = opcode.parse::<Int>().unwrap();
        let a = a.parse::<Int>().unwrap();
        let b = b.parse::<Int>().unwrap();
        let c = c.parse::<Int>().unwrap();
        let op = assignments[&opcode];
        assert!(
            op.evaluate(&mut regs, a, b, c),
            "Opcode {op:?} failed to run"
        );
    }
    regs.get(0).unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
