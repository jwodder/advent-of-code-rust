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
    let mut program = input.parse::<Intcode>();
    let output = program.run(VecDeque::from([5]));
    match output.len() {
        1 => output[0],
        n => panic!("Got {n} outputs, expected 1"),
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // input == 8:
    #[case("3,9,8,9,10,9,4,9,99,-1,8", 8, 1)]
    #[case("3,3,1108,-1,8,3,4,3,99", 8, 1)]
    #[case("3,9,8,9,10,9,4,9,99,-1,8", 42, 0)]
    #[case("3,3,1108,-1,8,3,4,3,99", 42, 0)]
    // input < 8:
    #[case("3,9,7,9,10,9,4,9,99,-1,8", 6, 1)]
    #[case("3,3,1107,-1,8,3,4,3,99", 6, 1)]
    #[case("3,9,7,9,10,9,4,9,99,-1,8", 8, 0)]
    #[case("3,3,1107,-1,8,3,4,3,99", 8, 0)]
    #[case("3,9,7,9,10,9,4,9,99,-1,8", 42, 0)]
    #[case("3,3,1107,-1,8,3,4,3,99", 42, 0)]
    fn test_cmp8(#[case] s: &str, #[case] input: i32, #[case] output: i32) {
        let mut program = s.parse::<Intcode>().unwrap();
        assert_eq!(program.run(VecDeque::from([input])), [output]);
    }

    #[rstest]
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0)]
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0)]
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1, 1)]
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1)]
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 42, 1)]
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 42, 1)]
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -23, 1)]
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -23, 1)]
    fn test_boolify(#[case] s: &str, #[case] input: i32, #[case] output: i32) {
        let mut program = s.parse::<Intcode>().unwrap();
        assert_eq!(program.run(VecDeque::from([input])), [output]);
    }

    #[test]
    fn test_example3a() {
        let mut program = concat!(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
            "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
            "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        )
        .parse::<Intcode>()
        .unwrap();
        assert_eq!(program.run(VecDeque::from([6])), [999]);
    }

    #[test]
    fn test_example3b() {
        let mut program = concat!(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
            "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
            "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        )
        .parse::<Intcode>()
        .unwrap();
        assert_eq!(program.run(VecDeque::from([8])), [1000]);
    }

    #[test]
    fn test_example3c() {
        let mut program = concat!(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
            "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
            "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        )
        .parse::<Intcode>()
        .unwrap();
        assert_eq!(program.run(VecDeque::from([42])), [1001]);
    }
}
