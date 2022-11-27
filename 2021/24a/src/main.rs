use adventutil::Input;
use std::collections::VecDeque;
use std::iter::FusedIterator;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn run<I: IntoIterator<Item = i32>>(&self, input: I) -> Output {
        let mut state = State {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input: input.into_iter().collect(),
        };
        for i in &self.instructions {
            i.operate(&mut state);
        }
        state.into()
    }
}

impl FromIterator<Instruction> for Program {
    fn from_iter<I: IntoIterator<Item = Instruction>>(iter: I) -> Program {
        Program {
            instructions: iter.into_iter().collect(),
        }
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Program, ParseError> {
        Ok(Program {
            instructions: s
                .lines()
                .map(|l| l.parse::<Instruction>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    input: VecDeque<i32>,
}

impl State {
    fn get_var(&self, var: Variable) -> i32 {
        match var {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn resolve(&self, value: VarOrNum) -> i32 {
        match value {
            VarOrNum::Var(v) => self.get_var(v),
            VarOrNum::Num(n) => n,
        }
    }

    fn set_var(&mut self, var: Variable, value: i32) {
        match var {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
        }
    }

    fn get_input(&mut self) -> i32 {
        self.input.pop_front().expect("Out of input")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Output {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl From<State> for Output {
    fn from(st: State) -> Output {
        Output {
            w: st.w,
            x: st.x,
            y: st.y,
            z: st.z,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, VarOrNum),
    Mul(Variable, VarOrNum),
    Div(Variable, VarOrNum),
    Mod(Variable, VarOrNum),
    Eql(Variable, VarOrNum),
}

impl Instruction {
    fn operate(&self, state: &mut State) {
        match self {
            Instruction::Inp(v) => {
                let value = state.get_input();
                state.set_var(*v, value);
            }
            Instruction::Add(v, arg) => state.set_var(*v, state.get_var(*v) + state.resolve(*arg)),
            Instruction::Mul(v, arg) => state.set_var(*v, state.get_var(*v) * state.resolve(*arg)),
            Instruction::Div(v, arg) => state.set_var(*v, state.get_var(*v) / state.resolve(*arg)),
            Instruction::Mod(v, arg) => state.set_var(*v, state.get_var(*v) % state.resolve(*arg)),
            Instruction::Eql(v, arg) => {
                state.set_var(*v, i32::from(state.get_var(*v) == state.resolve(*arg)))
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut words = s.split_whitespace();
        match words.next() {
            Some("inp") => {
                let var = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                Ok(Instruction::Inp(var))
            }
            Some("add") => {
                let left = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                let right = words.next().ok_or(ParseError::Short)?.parse::<VarOrNum>()?;
                Ok(Instruction::Add(left, right))
            }
            Some("mul") => {
                let left = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                let right = words.next().ok_or(ParseError::Short)?.parse::<VarOrNum>()?;
                Ok(Instruction::Mul(left, right))
            }
            Some("div") => {
                let left = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                let right = words.next().ok_or(ParseError::Short)?.parse::<VarOrNum>()?;
                Ok(Instruction::Div(left, right))
            }
            Some("mod") => {
                let left = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                let right = words.next().ok_or(ParseError::Short)?.parse::<VarOrNum>()?;
                Ok(Instruction::Mod(left, right))
            }
            Some("eql") => {
                let left = words.next().ok_or(ParseError::Short)?.parse::<Variable>()?;
                let right = words.next().ok_or(ParseError::Short)?.parse::<VarOrNum>()?;
                Ok(Instruction::Eql(left, right))
            }
            Some(s) => Err(ParseError::UnknownInstruction(s.to_string())),
            None => Err(ParseError::Short),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Variable {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Variable, ParseError> {
        match s {
            "w" => Ok(Variable::W),
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            s => Err(ParseError::InvalidVariable(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VarOrNum {
    Var(Variable),
    Num(i32),
}

impl FromStr for VarOrNum {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<VarOrNum, ParseError> {
        match s.parse::<Variable>() {
            Ok(v) => Ok(VarOrNum::Var(v)),
            Err(_) => Ok(VarOrNum::Num(s.parse::<i32>()?)),
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid variable name: {0:?}")]
    InvalidVariable(String),
    #[error("Unknown instruction: {0:?}")]
    UnknownInstruction(String),
    #[error("Input ended abruptly")]
    Short,
    #[error("Invalid integer: {0}")]
    InvalidInteger(#[from] ParseIntError),
}

fn model_numbers(digits: usize) -> ModelNumbers {
    ModelNumbers::new(digits)
}

enum ModelNumbers {
    Running(Vec<i32>),
    Done,
}

impl ModelNumbers {
    fn new(digits: usize) -> ModelNumbers {
        ModelNumbers::Running(vec![9; digits])
    }
}

impl Iterator for ModelNumbers {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        match self {
            ModelNumbers::Running(digits) => {
                let r = digits.clone();
                let mut done = true;
                for i in (0..digits.len()).rev() {
                    digits[i] -= 1;
                    if digits[i] == 0 {
                        digits[i] = 9;
                    } else {
                        done = false;
                        break;
                    }
                }
                if done {
                    *self = ModelNumbers::Done;
                }
                Some(r)
            }
            ModelNumbers::Done => None,
        }
    }
}

impl FusedIterator for ModelNumbers {}

fn main() {
    let program = Input::from_env().parse::<Program>();
    for num in model_numbers(14) {
        if program.run(num.clone()).z == 0 {
            println!(
                "{}",
                num.into_iter().map(|d| d.to_string()).collect::<String>()
            );
            return;
        }
    }
    panic!("No valid model numbers found");
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use Instruction::*;
    use VarOrNum::*;
    use Variable::*;

    #[rstest]
    #[case("inp w", Inp(W))]
    #[case("inp x", Inp(X))]
    #[case("inp y", Inp(Y))]
    #[case("inp z", Inp(Z))]
    #[case("mul x -1", Mul(X, Num(-1)))]
    #[case("mul z 3", Mul(Z, Num(3)))]
    #[case("eql z x", Eql(Z, Var(X)))]
    #[case("add z w", Add(Z, Var(W)))]
    #[case("mod z 2", Mod(Z, Num(2)))]
    #[case("div w 2", Div(W, Num(2)))]
    fn test_parse_instruction(#[case] s: &str, #[case] instruction: Instruction) {
        assert_eq!(s.parse::<Instruction>().unwrap(), instruction);
    }

    #[rstest]
    #[case(42, -42)]
    #[case(-23, 23)]
    #[case(0, 0)]
    fn test_negate_program(#[case] input: i32, #[case] x: i32) {
        let program = "inp x\nmul x -1".parse::<Program>().unwrap();
        let output = program.run([input]);
        assert_eq!(output.x, x);
    }

    #[rstest]
    #[case([2, 6], 1)]
    #[case([2, 5], 0)]
    #[case([2, 7], 0)]
    #[case([6, 2], 0)]
    fn test_triple_eq_program(#[case] input: [i32; 2], #[case] z: i32) {
        let program = "inp z\ninp x\nmul z 3\neql z x".parse::<Program>().unwrap();
        let output = program.run(input);
        assert_eq!(output.z, z);
    }

    #[rstest]
    #[case(0, Output {w: 0, x: 0, y: 0, z: 0})]
    #[case(1, Output {w: 0, x: 0, y: 0, z: 1})]
    #[case(2, Output {w: 0, x: 0, y: 1, z: 0})]
    #[case(4, Output {w: 0, x: 1, y: 0, z: 0})]
    #[case(5, Output {w: 0, x: 1, y: 0, z: 1})]
    #[case(6, Output {w: 0, x: 1, y: 1, z: 0})]
    #[case(8, Output {w: 1, x: 0, y: 0, z: 0})]
    #[case(10, Output {w: 1, x: 0, y: 1, z: 0})]
    #[case(15, Output {w: 1, x: 1, y: 1, z: 1})]
    #[case(255, Output {w: 1, x: 1, y: 1, z: 1})]
    #[case(256, Output {w: 0, x: 0, y: 0, z: 0})]
    fn test_nibble_bits_program(#[case] input: i32, #[case] expected: Output) {
        let program = concat!(
            "inp w\n",
            "add z w\n",
            "mod z 2\n",
            "div w 2\n",
            "add y w\n",
            "mod y 2\n",
            "div w 2\n",
            "add x w\n",
            "mod x 2\n",
            "div w 2\n",
            "mod w 2\n",
        )
        .parse::<Program>()
        .unwrap();
        let output = program.run([input]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_model_numbers_one_digit() {
        let mut iter = model_numbers(1);
        assert_eq!(iter.next().unwrap(), [9]);
        assert_eq!(iter.next().unwrap(), [8]);
        assert_eq!(iter.next().unwrap(), [7]);
        assert_eq!(iter.next().unwrap(), [6]);
        assert_eq!(iter.next().unwrap(), [5]);
        assert_eq!(iter.next().unwrap(), [4]);
        assert_eq!(iter.next().unwrap(), [3]);
        assert_eq!(iter.next().unwrap(), [2]);
        assert_eq!(iter.next().unwrap(), [1]);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_model_numbers_two_digits_len() {
        let iter = model_numbers(2);
        assert_eq!(iter.count(), 81);
    }

    #[test]
    fn test_model_numbers_two_digits_head() {
        let mut iter = model_numbers(2);
        assert_eq!(iter.next().unwrap(), [9, 9]);
        assert_eq!(iter.next().unwrap(), [9, 8]);
        assert_eq!(iter.next().unwrap(), [9, 7]);
        assert_eq!(iter.next().unwrap(), [9, 6]);
        assert_eq!(iter.next().unwrap(), [9, 5]);
        assert_eq!(iter.next().unwrap(), [9, 4]);
        assert_eq!(iter.next().unwrap(), [9, 3]);
        assert_eq!(iter.next().unwrap(), [9, 2]);
        assert_eq!(iter.next().unwrap(), [9, 1]);
        assert_eq!(iter.next().unwrap(), [8, 9]);
        assert_eq!(iter.next().unwrap(), [8, 8]);
    }
}
