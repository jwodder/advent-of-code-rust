use adventutil::Input;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Santa {
    pub x: i32,
    pub y: i32,
}

impl Santa {
    fn new() -> Santa {
        Santa { x: 0, y: 0 }
    }

    fn domove(&mut self, c: char) {
        match c {
            '^' => self.y += 1,
            '>' => self.x += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            _ => (),
        }
    }
}

fn solve(input: Input) -> usize {
    let mut seen = HashSet::new();
    let start = Santa::new();
    seen.insert(start);
    let mut locs = [start, start];
    let mut index = 0;
    for c in input.read().chars() {
        locs[index].domove(c);
        seen.insert(locs[index]);
        index = 1 - index;
    }
    seen.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_visited(#[case] s: &'static str, #[case] qty: usize) {
        let input = Input::from(s);
        assert_eq!(solve(input), qty);
    }
}
