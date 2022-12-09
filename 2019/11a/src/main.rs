use adventutil::gridgeom::{Point, Vector};
use adventutil::intcode::{Intcode, IntcodeIO};
use adventutil::Input;
use std::collections::HashSet;

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
            white: HashSet::new(),
            painted: HashSet::new(),
            pos: Point::ORIGIN,
            facing: Vector::NORTH,
            on_paint: true,
        }
    }

    fn painted(&self) -> usize {
        self.painted.len()
    }
}

impl IntcodeIO for Hull {
    fn recv(&mut self) -> i64 {
        i64::from(self.white.contains(&self.pos))
    }

    fn send(&mut self, value: i64) {
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
}

fn solve(input: Input) -> usize {
    let mut program = input.parse::<Intcode>();
    let mut hull = Hull::new();
    program.run(&mut hull);
    hull.painted()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
