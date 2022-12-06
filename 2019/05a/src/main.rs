use adventutil::Input;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

struct Intcode(Vec<i32>);

impl Intcode {
    fn run(&mut self, mut input: VecDeque<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut output = Vec::new();
        while i < self.0.len() {
            match self.0[i] % 100 {
                1 => {
                    let params = self.get_params(i, 3);
                    self.write_to_param(
                        params[2],
                        self.eval_param(params[0]) + self.eval_param(params[1]),
                    );
                    i += 4;
                }
                2 => {
                    let params = self.get_params(i, 3);
                    self.write_to_param(
                        params[2],
                        self.eval_param(params[0]) * self.eval_param(params[1]),
                    );
                    i += 4;
                }
                3 => {
                    let params = self.get_params(i, 1);
                    let value = input.pop_front().expect("Out of input");
                    self.write_to_param(params[0], value);
                    i += 2;
                }
                4 => {
                    let params = self.get_params(i, 1);
                    output.push(self.eval_param(params[0]));
                    i += 2;
                }
                99 => return output,
                n => panic!("Invalid opcode {n}"),
            }
        }
        output
    }

    fn get_params(&self, op_index: usize, qty: usize) -> Vec<Parameter> {
        let mut params = Vec::with_capacity(qty);
        let mut opcode = self.0[op_index] / 100;
        for i in (op_index + 1)..(op_index + 1 + qty) {
            match opcode % 10 {
                0 => params.push(Parameter::Address(
                    usize::try_from(self.0[i]).expect("Address out of usize range"),
                )),
                1 => params.push(Parameter::Value(self.0[i])),
                n => panic!("Invalid parameter mode {n}"),
            }
            opcode /= 10;
        }
        params
    }

    fn eval_param(&self, param: Parameter) -> i32 {
        match param {
            Parameter::Address(addr) => self.0[addr],
            Parameter::Value(value) => value,
        }
    }

    fn write_to_param(&mut self, param: Parameter, value: i32) {
        match param {
            Parameter::Address(addr) => self.0[addr] = value,
            Parameter::Value(_) => panic!("Cannot set immediate-mode parameter"),
        }
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Intcode, ParseIntError> {
        Ok(Intcode(
            s.split(',')
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Parameter {
    Address(usize),
    Value(i32),
}

fn solve(input: Input) -> i32 {
    let mut program = input.parse::<Intcode>();
    let output = program.run(VecDeque::from([1]));
    let lasti = output.len() - 1;
    if output[0..lasti].iter().all(|&i| i == 0) {
        output[lasti]
    } else {
        panic!("Tests failed")
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut program = "3,0,4,0,99".parse::<Intcode>().unwrap();
        let output = program.run(VecDeque::from([42]));
        assert_eq!(output, [42]);
    }

    #[test]
    fn test_example2() {
        let mut program = "1002,4,3,4,33".parse::<Intcode>().unwrap();
        let output = program.run(VecDeque::new());
        assert_eq!(output, []);
        assert_eq!(program.0, [1002, 4, 3, 4, 99]);
    }
}
