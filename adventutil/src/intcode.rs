use std::collections::{HashMap, VecDeque};
use std::num::{ParseIntError, TryFromIntError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Intcode {
    pub program: Vec<i64>,
    pub extra_memory: HashMap<usize, i64>,
    pub relative_base: i64,
}

impl Intcode {
    pub fn new(program: Vec<i64>) -> Intcode {
        Intcode {
            program,
            extra_memory: HashMap::new(),
            relative_base: 0,
        }
    }

    pub fn run<IO: IntcodeIO>(&mut self, io: &mut IO) -> Result<(), IntcodeError> {
        let mut i = 0;
        loop {
            match self.get(i) % 100 {
                1 => {
                    let params = self.get_params(i, 3)?;
                    self.write_to_param(
                        params[2],
                        self.eval_param(params[0]) + self.eval_param(params[1]),
                    )?;
                    i += 4;
                }
                2 => {
                    let params = self.get_params(i, 3)?;
                    self.write_to_param(
                        params[2],
                        self.eval_param(params[0]) * self.eval_param(params[1]),
                    )?;
                    i += 4;
                }
                3 => {
                    let params = self.get_params(i, 1)?;
                    let value = io.recv();
                    self.write_to_param(params[0], value)?;
                    i += 2;
                }
                4 => {
                    let params = self.get_params(i, 1)?;
                    io.send(self.eval_param(params[0]));
                    i += 2;
                }
                5 => {
                    let params = self.get_params(i, 2)?;
                    if self.eval_param(params[0]) != 0 {
                        i = value2addr(self.eval_param(params[1]))?;
                    } else {
                        i += 3;
                    }
                }
                6 => {
                    let params = self.get_params(i, 2)?;
                    if self.eval_param(params[0]) == 0 {
                        i = value2addr(self.eval_param(params[1]))?;
                    } else {
                        i += 3;
                    }
                }
                7 => {
                    let params = self.get_params(i, 3)?;
                    self.write_to_param(
                        params[2],
                        i64::from(self.eval_param(params[0]) < self.eval_param(params[1])),
                    )?;
                    i += 4;
                }
                8 => {
                    let params = self.get_params(i, 3)?;
                    self.write_to_param(
                        params[2],
                        i64::from(self.eval_param(params[0]) == self.eval_param(params[1])),
                    )?;
                    i += 4;
                }
                9 => {
                    let params = self.get_params(i, 1)?;
                    self.relative_base += self.eval_param(params[0]);
                    i += 2;
                }
                99 => return Ok(()),
                n => {
                    return Err(IntcodeError::InvalidOpcode {
                        opcode: n,
                        index: i,
                    })
                }
            }
        }
    }

    pub fn get(&self, i: usize) -> i64 {
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

    fn get_params(&self, op_index: usize, qty: usize) -> Result<Vec<Parameter>, IntcodeError> {
        let mut params = Vec::with_capacity(qty);
        let mut opcode = self.get(op_index) / 100;
        for i in (op_index + 1)..(op_index + 1 + qty) {
            match opcode % 10 {
                0 => params.push(Parameter::Address(value2addr(self.get(i))?)),
                1 => params.push(Parameter::Value(self.get(i))),
                2 => params.push(Parameter::Address(value2addr(
                    self.get(i) + self.relative_base,
                )?)),
                n => {
                    return Err(IntcodeError::InvalidParamMode {
                        mode: n,
                        index: op_index,
                    })
                }
            }
            opcode /= 10;
        }
        Ok(params)
    }

    fn eval_param(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Address(addr) => self.get(addr),
            Parameter::Value(value) => value,
        }
    }

    fn write_to_param(&mut self, param: Parameter, value: i64) -> Result<(), IntcodeError> {
        match param {
            Parameter::Address(addr) => self.set(addr, value),
            Parameter::Value(_) => return Err(IntcodeError::WriteToImmediate),
        }
        Ok(())
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

#[derive(Debug, Error)]
pub enum IntcodeError {
    #[error("Invalid opcode {opcode} at program index {index}")]
    InvalidOpcode { opcode: i64, index: usize },
    #[error("Invalid parameter mode {mode} in opcode at program index {index}")]
    InvalidParamMode { mode: i64, index: usize },
    #[error("Address value {value} out of usize range: {source}")]
    Value2AddrError { value: i64, source: TryFromIntError },
    #[error("Cannot write to immediate-mode parameter")]
    WriteToImmediate,
}

pub trait IntcodeIO {
    /// Provide an input value to the Intcode program
    fn recv(&mut self) -> i64;

    /// Process an output value from the Intcode program
    fn send(&mut self, value: i64);
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct VecIO {
    pub input: VecDeque<i64>,
    pub output: Vec<i64>,
}

impl<const N: usize> From<[i64; N]> for VecIO {
    fn from(data: [i64; N]) -> VecIO {
        VecIO {
            input: VecDeque::from(data),
            output: Vec::new(),
        }
    }
}

impl IntcodeIO for VecIO {
    fn recv(&mut self) -> i64 {
        self.input.pop_front().expect("Out of input")
    }

    fn send(&mut self, value: i64) {
        self.output.push(value);
    }
}

fn value2addr(value: i64) -> Result<usize, IntcodeError> {
    usize::try_from(value).map_err(|source| IntcodeError::Value2AddrError { value, source })
}
