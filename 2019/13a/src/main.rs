use adventutil::intcode::{Intcode, IntcodeIO};
use adventutil::Input;
use std::collections::HashMap;

struct Screen {
    drawn: HashMap<(i64, i64), i64>,
    state: OutputState,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            drawn: HashMap::new(),
            state: OutputState::Start,
        }
    }

    fn blocks(self) -> usize {
        self.drawn.into_values().filter(|&i| i == 2).count()
    }
}

impl IntcodeIO for Screen {
    fn recv(&mut self) -> i64 {
        panic!("No input to provide")
    }

    fn send(&mut self, value: i64) {
        self.state = match self.state {
            OutputState::Start => OutputState::XSent(value),
            OutputState::XSent(x) => OutputState::XYSent(x, value),
            OutputState::XYSent(x, y) => {
                self.drawn.insert((x, y), value);
                OutputState::Start
            }
        };
    }
}

enum OutputState {
    Start,
    XSent(i64),
    XYSent(i64, i64),
}

fn solve(input: Input) -> usize {
    let mut program = input.parse::<Intcode>();
    let mut screen = Screen::new();
    program.run(&mut screen).unwrap();
    screen.blocks()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
