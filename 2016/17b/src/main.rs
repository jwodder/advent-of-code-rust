use adventutil::maxtracker::MaxTracker;
use adventutil::Input;
use md5::{Digest, Md5};
use std::collections::{hash_map::Entry, HashMap, HashSet};

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
    let mut distances = HashMap::from([(start, 0)]);
    let mut longest_path = MaxTracker::new();
    loop {
        let Some((current, dist)) = distances
            .iter()
            .filter(|&(k, _)| !visited.contains(k))
            .min_by_key(|&(_, &dist)| dist)
            .map(|(k, &dist)| (k.clone(), dist))
        else {
            return longest_path.get().unwrap();
        };
        visited.insert(current.clone());
        if current.at_end() {
            distances.remove(&current);
            longest_path.add(dist);
            continue;
        }
        for p in current.next_states() {
            if !visited.contains(&p) {
                let newdist = dist + 1;
                match distances.entry(p) {
                    Entry::Vacant(e) => {
                        e.insert(newdist);
                    }
                    Entry::Occupied(mut e) if *e.get() > newdist => {
                        e.insert(newdist);
                    }
                    _ => (),
                }
            }
        }
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
    #[case("ihgpwlah", 370)]
    #[case("kglvqrro", 492)]
    #[case("ulqzkmiv", 830)]
    fn test_example(#[case] passcode: &'static str, #[case] longest_path: usize) {
        let input = Input::from(passcode);
        assert_eq!(solve(input), longest_path);
    }
}
