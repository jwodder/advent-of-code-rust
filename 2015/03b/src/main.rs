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

fn visited(s: &str) -> usize {
    let mut seen = HashSet::new();
    let start = Santa::new();
    seen.insert(start);
    let mut locs = [start, start];
    let mut index = 0;
    for c in s.chars() {
        locs[index].domove(c);
        seen.insert(locs[index]);
        index = 1 - index;
    }
    seen.len()
}

fn main() {
    println!("{}", visited(&Input::from_env().read()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_visited(#[case] s: &str, #[case] qty: usize) {
        assert_eq!(visited(s), qty);
    }
}
