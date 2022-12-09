use adventutil::grid::{Grid, GridBounds};
use adventutil::gridgeom::{Point, PointBounds, Vector};
use adventutil::Input;
use std::collections::{HashMap, HashSet};
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

    fn run(&mut self, hull: &mut Hull) {
        let mut i = 0;
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
                    let value = hull.get_input();
                    self.write_to_param(params[0], value);
                    i += 2;
                }
                4 => {
                    let params = self.get_params(i, 1);
                    hull.output(self.eval_param(params[0]));
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
                99 => return,
                n => panic!("Invalid opcode {n}"),
            }
        }
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

struct Hull {
    white: HashSet<Point>,
    painted: HashSet<Point>,
    pos: Point,
    facing: Vector,
    on_paint: bool,
}

impl Hull {
    fn new() -> Self {
        Hull {
            white: HashSet::from([Point::ORIGIN]),
            painted: HashSet::new(),
            pos: Point::ORIGIN,
            facing: Vector::NORTH,
            on_paint: true,
        }
    }

    fn get_input(&self) -> i64 {
        i64::from(self.white.contains(&self.pos))
    }

    fn output(&mut self, value: i64) {
        if self.on_paint {
            self.painted.insert(self.pos);
            if value == 0 {
                self.white.remove(&self.pos);
            } else if value == 1 {
                self.white.insert(self.pos);
            } else {
                panic!("Got unexpected output from program: {value}");
            }
        } else {
            if value == 0 {
                self.facing = self.facing.turn_left();
            } else if value == 1 {
                self.facing = self.facing.turn_right();
            } else {
                panic!("Got unexpected output from program: {value}");
            }
            self.pos += self.facing;
        }
        self.on_paint = !self.on_paint;
    }

    fn painting(&self) -> String {
        let pbounds = PointBounds::for_points(self.painted.iter().copied()).unwrap();
        let Point { x: ulx, y: uly } = pbounds.ulcorner();
        let grbounds = GridBounds::new(
            usize::try_from(pbounds.height()).unwrap(),
            usize::try_from(pbounds.width()).unwrap(),
        );
        Grid::<char>::from_fn(grbounds, |(y, x)| {
            let y = i32::try_from(y).unwrap();
            let x = i32::try_from(x).unwrap();
            if self.white.contains(&Point {
                x: ulx + x,
                y: uly - y,
            }) {
                '#'
            } else {
                '.'
            }
        })
        .to_string()
    }
}

fn solve(input: Input) -> String {
    let mut program = input.parse::<Intcode>();
    let mut hull = Hull::new();
    program.run(&mut hull);
    hull.painting()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
