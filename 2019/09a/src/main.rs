use adventutil::Input;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

struct Intcode {
    program: Vec<i64>,
    extra_memory: HashMap<usize, i64>,
    relative_base: i64,
}

impl Intcode {
    fn new(program: Vec<i64>) -> Intcode {
        Intcode {
            program,
            extra_memory: HashMap::new(),
            relative_base: 0,
        }
    }

    fn get(&self, i: usize) -> i64 {
        *self
            .program
            .get(i)
            .or_else(|| self.extra_memory.get(&i))
            .unwrap_or(&0)
    }

    fn set(&mut self, i: usize, value: i64) {
        if i < self.program.len() {
            self.program[i] = value;
        } else {
            self.extra_memory.insert(i, value);
        }
    }

    fn run(&mut self, mut input: VecDeque<i64>) -> Vec<i64> {
        let mut i = 0;
        let mut output = Vec::new();
        while i < self.program.len() {
            match self.program[i] % 100 {
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
                        i64::from(self.eval_param(params[0]) < self.eval_param(params[1])),
                    );
                    i += 4;
                }
                8 => {
                    let params = self.get_params(i, 3);
                    self.write_to_param(
                        params[2],
                        i64::from(self.eval_param(params[0]) == self.eval_param(params[1])),
                    );
                    i += 4;
                }
                9 => {
                    let params = self.get_params(i, 1);
                    self.relative_base += self.eval_param(params[0]);
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
        let mut opcode = self.program[op_index] / 100;
        for i in (op_index + 1)..(op_index + 1 + qty) {
            match opcode % 10 {
                0 => params.push(Parameter::Address(
                    usize::try_from(self.program[i]).expect("Address out of usize range"),
                )),
                1 => params.push(Parameter::Value(self.program[i])),
                2 => params.push(Parameter::Address(
                    usize::try_from(self.program[i] + self.relative_base)
                        .expect("Address out of usize range"),
                )),
                n => panic!("Invalid parameter mode {n}"),
            }
            opcode /= 10;
        }
        params
    }

    fn eval_param(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Address(addr) => self.get(addr),
            Parameter::Value(value) => value,
        }
    }

    fn write_to_param(&mut self, param: Parameter, value: i64) {
        match param {
            Parameter::Address(addr) => self.set(addr, value),
            Parameter::Value(_) => panic!("Cannot set immediate-mode parameter"),
        }
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Intcode, ParseIntError> {
        Ok(Intcode::new(
            s.split(',')
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Parameter {
    Address(usize),
    Value(i64),
}

fn solve(input: Input) -> i64 {
    let mut program = input.parse::<Intcode>();
    let output = program.run(VecDeque::from([1]));
    match output.len() {
        1 => output[0],
        n => panic!("Got {n} outputs, expected 1"),
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
        let mut program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .parse::<Intcode>()
            .unwrap();
        assert_eq!(
            program.run(VecDeque::new()),
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_example2() {
        let mut program = "1102,34915192,34915192,7,4,7,99,0"
            .parse::<Intcode>()
            .unwrap();
        let output = program.run(VecDeque::new());
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].to_string().len(), 16);
    }

    #[test]
    fn test_example3() {
        let mut program = "104,1125899906842624,99".parse::<Intcode>().unwrap();
        assert_eq!(program.run(VecDeque::new()), [1125899906842624]);
    }
}
