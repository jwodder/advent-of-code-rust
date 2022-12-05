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
                    let modes = get_modes(self.0[i], 3);
                    self.set_param_at(
                        i + 3,
                        self.get_param_at(i + 1, modes[0]) + self.get_param_at(i + 2, modes[1]),
                        modes[2],
                    );
                    i += 4;
                }
                2 => {
                    let modes = get_modes(self.0[i], 3);
                    self.set_param_at(
                        i + 3,
                        self.get_param_at(i + 1, modes[0]) * self.get_param_at(i + 2, modes[1]),
                        modes[2],
                    );
                    i += 4;
                }
                3 => {
                    let modes = get_modes(self.0[i], 1);
                    let value = input.pop_front().expect("Out of input");
                    self.set_param_at(i + 1, value, modes[0]);
                    i += 2;
                }
                4 => {
                    let modes = get_modes(self.0[i], 1);
                    output.push(self.get_param_at(i + 1, modes[0]));
                    i += 2;
                }
                99 => return output,
                n => panic!("Invalid opcode {n}"),
            }
        }
        output
    }

    fn get_param_at(&self, address: usize, mode: ParamMode) -> i32 {
        let param = self.0[address];
        match mode {
            ParamMode::Position => *self
                .0
                .get(usize::try_from(param).expect("Parameter out of usize range"))
                .expect("Parameter out of bounds"),
            ParamMode::Immediate => param,
        }
    }

    fn set_param_at(&mut self, address: usize, value: i32, mode: ParamMode) {
        let param = self.0[address];
        match mode {
            ParamMode::Position => {
                *self
                    .0
                    .get_mut(usize::try_from(param).expect("Parameter out of usize range"))
                    .expect("Parameter out of bounds") = value
            }
            ParamMode::Immediate => panic!("Cannot set immediate-mode parameter"),
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
enum ParamMode {
    Position,
    Immediate,
}

fn get_modes(mut opcode: i32, qty: usize) -> Vec<ParamMode> {
    let mut modes = Vec::with_capacity(qty);
    opcode /= 100;
    for _ in 0..qty {
        match opcode % 10 {
            0 => modes.push(ParamMode::Position),
            1 => modes.push(ParamMode::Immediate),
            n => panic!("Invalid parameter mode {n}"),
        }
        opcode /= 10;
    }
    modes
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
