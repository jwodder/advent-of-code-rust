use adventutil::Input;
use itertools::Itertools;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
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
                5 => {
                    let params = self.get_params(i, 2);
                    if self.eval_param(params[0]) != 0 {
                        i = usize::try_from(self.eval_param(params[1]))
                            .expect("Parameter out of usize range");
                    } else {
                        i += 3;
                    }
                }
                6 => {
                    let params = self.get_params(i, 2);
                    if self.eval_param(params[0]) == 0 {
                        i = usize::try_from(self.eval_param(params[1]))
                            .expect("Parameter out of usize range");
                    } else {
                        i += 3;
                    }
                }
                7 => {
                    let params = self.get_params(i, 3);
                    self.write_to_param(
                        params[2],
                        i32::from(self.eval_param(params[0]) < self.eval_param(params[1])),
                    );
                    i += 4;
                }
                8 => {
                    let params = self.get_params(i, 3);
                    self.write_to_param(
                        params[2],
                        i32::from(self.eval_param(params[0]) == self.eval_param(params[1])),
                    );
                    i += 4;
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
    let program = input.parse::<Intcode>();
    (0..5)
        .permutations(5)
        .map(|perm| {
            perm.into_iter().fold(0, |input, phase| {
                program.clone().run(VecDeque::from([phase, input]))[0]
            })
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    #[case(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        54321
    )]
    #[case("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210)]
    fn test_solve(#[case] program: &'static str, #[case] thrust: i32) {
        let input = Input::from(program);
        assert_eq!(solve(input), thrust);
    }
}
