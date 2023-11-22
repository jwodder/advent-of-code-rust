use adventutil::intcode::{Intcode, IntcodeIO};
use adventutil::Input;
use itertools::Itertools;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;

struct ChannelIO {
    receiver: Receiver<i64>,
    sender: Sender<i64>,
}

impl IntcodeIO for ChannelIO {
    fn send(&mut self, value: i64) {
        self.sender.send(value).unwrap();
    }

    fn recv(&mut self) -> i64 {
        self.receiver.recv().unwrap()
    }
}

fn feedback_loop(program: &Intcode, phases: Vec<i64>) -> i64 {
    let (out1, in2) = channel();
    let (out2, in3) = channel();
    let (out3, in4) = channel();
    let (out4, in5) = channel();
    let (out5, loop_in) = channel();
    let (loop_out, in1) = channel();
    loop_out.send(phases[0]).unwrap();
    loop_out.send(0).unwrap();
    out1.send(phases[1]).unwrap();
    out2.send(phases[2]).unwrap();
    out3.send(phases[3]).unwrap();
    out4.send(phases[4]).unwrap();
    for mut io in [
        ChannelIO {
            receiver: in1,
            sender: out1,
        },
        ChannelIO {
            receiver: in2,
            sender: out2,
        },
        ChannelIO {
            receiver: in3,
            sender: out3,
        },
        ChannelIO {
            receiver: in4,
            sender: out4,
        },
        ChannelIO {
            receiver: in5,
            sender: out5,
        },
    ] {
        let mut code = program.clone();
        spawn(move || {
            if let Err(e) = code.run(&mut io) {
                eprintln!("ERROR: {e}");
            }
        });
    }
    let mut last = None;
    for value in &loop_in {
        let _ = last.insert(value);
        if loop_out.send(value).is_err() {
            break;
        }
    }
    last.unwrap()
}

fn solve(input: Input) -> i64 {
    let program = input.parse::<Intcode>();
    (5..10)
        .permutations(5)
        .map(|perm| feedback_loop(&program, perm))
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        139629729
    )]
    #[case("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", 18216)]
    fn test_solve(#[case] program: &'static str, #[case] thrust: i64) {
        let input = Input::from(program);
        assert_eq!(solve(input), thrust);
    }
}
