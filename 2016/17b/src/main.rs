use adventutil::Input;
use adventutil::maxtracker::MaxTracker;
use md5::{Digest, Md5};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    x: usize,
    y: usize,
    passcode: String,
    path: String,
}

impl State {
    fn start(passcode: String) -> State {
        State {
            x: 0,
            y: 0,
            passcode,
            path: String::new(),
        }
    }

    fn at_end(&self) -> bool {
        (self.x, self.y) == (3, 3)
    }

    fn next_states(&self) -> Vec<State> {
        let digest = hex::encode(Md5::digest(format!("{}{}", self.passcode, self.path)))
            .chars()
            .take(4)
            .collect::<Vec<_>>();
        let can_up = "bcdef".contains(digest[0]);
        let can_down = "bcdef".contains(digest[1]);
        let can_left = "bcdef".contains(digest[2]);
        let can_right = "bcdef".contains(digest[3]);
        let mut nexts = Vec::new();
        if can_up {
            nexts.extend(self.up());
        }
        if can_down {
            nexts.extend(self.down());
        }
        if can_left {
            nexts.extend(self.left());
        }
        if can_right {
            nexts.extend(self.right());
        }
        nexts
    }

    fn up(&self) -> Option<State> {
        Some(State {
            x: self.x,
            y: self.y.checked_sub(1)?,
            passcode: self.passcode.clone(),
            path: format!("{}U", self.path),
        })
    }

    fn down(&self) -> Option<State> {
        (self.y < 3).then(|| State {
            x: self.x,
            y: self.y + 1,
            passcode: self.passcode.clone(),
            path: format!("{}D", self.path),
        })
    }

    fn left(&self) -> Option<State> {
        Some(State {
            x: self.x.checked_sub(1)?,
            y: self.y,
            passcode: self.passcode.clone(),
            path: format!("{}L", self.path),
        })
    }

    fn right(&self) -> Option<State> {
        (self.x < 3).then(|| State {
            x: self.x + 1,
            y: self.y,
            passcode: self.passcode.clone(),
            path: format!("{}R", self.path),
        })
    }
}

fn solve(input: Input) -> usize {
    let passcode = input.read().trim().to_owned();
    let start = State::start(passcode);
    let mut visited = HashSet::new();
    let mut states = vec![start];
    let mut dist = 0;
    let mut longest_path = MaxTracker::new();
    while !states.is_empty() {
        let mut states2 = Vec::new();
        for current in states {
            if current.at_end() {
                longest_path.add(dist);
            } else {
                for p in current.next_states() {
                    if !visited.contains(&p) {
                        states2.push(p);
                    }
                }
                visited.insert(current);
            }
        }
        states = states2;
        dist += 1;
    }
    longest_path.get().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ihgpwlah", 370)]
    #[case("kglvqrro", 492)]
    #[case("ulqzkmiv", 830)]
    fn test_example(#[case] passcode: &'static str, #[case] longest_path: usize) {
        let input = Input::from(passcode);
        assert_eq!(solve(input), longest_path);
    }
}
