use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    register: String,
    adjustment: Adjustment,
    test_register: String,
    comparison: Comparison,
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        let register = parser.parse_to::<String, _>(Token::Whitespace)?;
        let adjust_op = parser.scan_to(Token::Whitespace)?;
        let adjust_by = parser.parse_to::<i32, _>(Token::Whitespace)?;
        let adjustment = match adjust_op {
            "inc" => Adjustment::Inc(adjust_by),
            "dec" => Adjustment::Dec(adjust_by),
            _ => return Err(ParseError::InvalidToken(adjust_op.to_owned())),
        };
        parser.skip("if ")?;
        let test_register = parser.parse_to::<String, _>(Token::Whitespace)?;
        let compare_op = parser.parse_to::<CompareOp, _>(Token::Whitespace)?;
        let right_side = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Instruction {
            register,
            adjustment,
            test_register,
            comparison: Comparison {
                op: compare_op,
                right_side,
            },
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Adjustment {
    Inc(i32),
    Dec(i32),
}

impl Adjustment {
    fn apply(&self, x: i32) -> i32 {
        match self {
            Adjustment::Inc(i) => x + i,
            Adjustment::Dec(i) => x - i,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Comparison {
    op: CompareOp,
    right_side: i32,
}

impl Comparison {
    fn compare(&self, testee: i32) -> bool {
        match self.op {
            CompareOp::Eq => testee == self.right_side,
            CompareOp::Neq => testee != self.right_side,
            CompareOp::Lt => testee < self.right_side,
            CompareOp::Le => testee <= self.right_side,
            CompareOp::Gt => testee > self.right_side,
            CompareOp::Ge => testee >= self.right_side,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompareOp {
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl std::str::FromStr for CompareOp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<CompareOp, ParseError> {
        match s {
            "==" => Ok(CompareOp::Eq),
            "!=" => Ok(CompareOp::Neq),
            "<" => Ok(CompareOp::Lt),
            "<=" => Ok(CompareOp::Le),
            ">" => Ok(CompareOp::Gt),
            ">=" => Ok(CompareOp::Ge),
            _ => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Registers(HashMap<String, i32>);

impl Registers {
    fn new() -> Registers {
        Registers(HashMap::new())
    }

    fn get(&self, register: &str) -> i32 {
        self.0.get(register).copied().unwrap_or_default()
    }

    fn adjust(&mut self, register: &str, adjustment: Adjustment) {
        let x = self.get(register);
        self.0.insert(register.to_owned(), adjustment.apply(x));
    }

    fn apply(&mut self, instruction: Instruction) {
        let testee = self.get(&instruction.test_register);
        if instruction.comparison.compare(testee) {
            self.adjust(&instruction.register, instruction.adjustment);
        }
    }

    fn max_value(self) -> i32 {
        self.0.into_values().max().unwrap_or_default()
    }
}

fn solve(input: Input) -> i32 {
    let mut registers = Registers::new();
    for instr in input.parse_lines::<Instruction>() {
        registers.apply(instr);
    }
    registers.max_value()
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
            "b inc 5 if a > 1\n",
            "a inc 1 if b < 5\n",
            "c dec -10 if a >= 1\n",
            "c inc -20 if c == 10\n",
        ));
        assert_eq!(solve(input), 1);
    }
}
